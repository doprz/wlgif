use crate::backend::{Backend, RecordConfig};
use crate::error::Error;
use crate::output;
use crate::region::Region;

use anyhow::{Context, Result};
use ashpd::desktop::PersistMode;
use ashpd::desktop::screencast::{CursorMode, Screencast, SourceType};
use gstreamer::prelude::*;
use std::os::fd::AsRawFd;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct XDGPortalBackend;

impl XDGPortalBackend {
    pub fn new() -> Self {
        Self
    }

    /// Check for required GStreamer plugins
    fn check_gstreamer() -> bool {
        gstreamer::init().is_ok()
            && gstreamer::ElementFactory::find("pipewiresrc").is_some()
            && gstreamer::ElementFactory::find("x264enc").is_some()
            && gstreamer::ElementFactory::find("mp4mux").is_some()
    }
}

const DEPS: &[&str] = &["ffmpeg"];

impl Backend for XDGPortalBackend {
    fn name(&self) -> &'static str {
        "xdg-desktop-portal"
    }

    fn is_available(&self) -> Result<()> {
        // TODO: This is shared among both backends
        for dep in DEPS {
            if which::which(dep).is_err() {
                return Err(Error::MissingDependency { dep }.into());
            }
        }

        if !Self::check_gstreamer() {
            return Err(Error::MissingGSPlugins.into());
        }

        Ok(())
    }

    fn record(&self, _region: Option<&Region>, output: &Path, config: &RecordConfig) -> Result<()> {
        // Portal doesn't support arbitrary region selection - it uses the portal's
        // own selection UI. Region parameter is ignored for now.
        // TODO: Implement crop filter for region support

        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .context("failed to create async runtime")?
            .block_on(record_async(output, config))
    }
}

async fn record_async(output: &Path, config: &RecordConfig) -> Result<()> {
    if !config.quiet {
        output::status("Requesting screen capture via portal...");
    }

    // Create screencast session
    let proxy = Screencast::new()
        .await
        .context("failed to connect to screencast portal")?;

    let session = proxy
        .create_session()
        .await
        .context("failed to create screencast session")?;

    // Request source selection (portal shows its own UI)
    proxy
        .select_sources(
            &session,
            CursorMode::Embedded, // Render cursor in the stream
            SourceType::Monitor | SourceType::Window,
            false, // Don't allow multiple sources
            None,  // No restore token
            PersistMode::DoNot,
        )
        .await
        .context("failed to select sources")?;

    // Start the screencast (portal may show confirmation dialog)
    let response = proxy
        .start(&session, None)
        .await
        .context("failed to start screencast")?
        .response()
        .context("screencast request was denied")?;

    let stream = response
        .streams()
        .first()
        .ok_or_else(|| Error::Recording("no streams returned from portal".into()))?;

    let node_id = stream.pipe_wire_node_id();

    if !config.quiet {
        output::status(&format!(
            "Recording PipeWire node {} ({}x{})",
            node_id,
            stream.size().map(|(w, _)| w).unwrap_or(0),
            stream.size().map(|(_, h)| h).unwrap_or(0),
        ));
    }

    // Get PipeWire fd for GStreamer
    let fd = proxy
        .open_pipe_wire_remote(&session)
        .await
        .context("failed to open PipeWire remote")?;

    // Record using GStreamer
    record_gstreamer(node_id, fd, output, config)?;

    Ok(())
}

fn record_gstreamer(
    node_id: u32,
    fd: std::os::fd::OwnedFd,
    output: &Path,
    config: &RecordConfig,
) -> Result<()> {
    gstreamer::init().context("failed to initialize GStreamer")?;

    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = Arc::clone(&stop);

    ctrlc::set_handler(move || {
        stop_clone.store(true, Ordering::SeqCst);
    })
    .context("failed to set signal handler")?;

    if !config.quiet {
        output::recording(config.duration);
    }

    // Build GStreamer pipeline:
    // pipewiresrc -> videoconvert -> x264enc -> mp4mux -> filesink
    let pipeline_str = format!(
        "pipewiresrc fd={fd} path={node_id} do-timestamp=true \
         ! videoconvert \
         ! videorate \
         ! video/x-raw,framerate={fps}/1 \
         ! x264enc tune=zerolatency speed-preset=ultrafast \
         ! mp4mux \
         ! filesink location={output}",
        fd = fd.as_raw_fd(),
        node_id = node_id,
        fps = config.fps,
        output = output.display(),
    );

    let pipeline = gstreamer::parse::launch(&pipeline_str)
        .context("failed to create GStreamer pipeline")?
        .downcast::<gstreamer::Pipeline>()
        .map_err(|_| anyhow::anyhow!("element is not a pipeline"))?;

    pipeline
        .set_state(gstreamer::State::Playing)
        .context("failed to start pipeline")?;

    let bus = pipeline.bus().context("pipeline has no bus")?;
    let poll = std::time::Duration::from_millis(50);
    let max_iters = if config.duration > 0.0 {
        ((config.duration * 1000.0) / poll.as_millis() as f32) as u64
    } else {
        u64::MAX
    };

    for _ in 0..max_iters {
        if stop.load(Ordering::SeqCst) {
            break;
        }

        // Check for pipeline errors
        if let Some(msg) = bus.timed_pop(gstreamer::ClockTime::from_mseconds(50)) {
            use gstreamer::MessageView;
            match msg.view() {
                MessageView::Error(err) => {
                    pipeline.set_state(gstreamer::State::Null).ok();
                    return Err(
                        Error::Recording(format!("GStreamer error: {}", err.error())).into(),
                    );
                }
                MessageView::Eos(_) => break,
                _ => {}
            }
        }
    }

    // Graceful shutdown: send EOS and wait for it to propagate
    pipeline.send_event(gstreamer::event::Eos::new());

    // Wait for EOS to be processed (max 5 seconds)
    for _ in 0..100 {
        if let Some(msg) = bus.timed_pop(gstreamer::ClockTime::from_mseconds(50))
            && let gstreamer::MessageView::Eos(_) = msg.view()
        {
            break;
        }
    }

    pipeline
        .set_state(gstreamer::State::Null)
        .context("failed to stop pipeline")?;

    Ok(())
}
