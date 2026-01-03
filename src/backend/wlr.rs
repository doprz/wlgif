use crate::backend::{Backend, RecordConfig};
use crate::error::Error;
use crate::output;
use crate::region::Region;
use anyhow::{Context, Result};
use std::{
    path::Path,
    process::{Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

pub struct WlrBackend;

impl WlrBackend {
    pub fn new() -> Self {
        Self
    }
}

const DEPS: &[&str] = &["slurp", "wf-recorder", "ffmpeg"];

impl Backend for WlrBackend {
    fn name(&self) -> &'static str {
        "wlroots"
    }

    fn is_available(&self) -> Result<()> {
        for dep in DEPS {
            if which::which(dep).is_err() {
                return Err(Error::MissingDependency { dep }.into());
            }
        }
        Ok(())
    }

    fn record(&self, region: Option<&Region>, output: &Path, config: &RecordConfig) -> Result<()> {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_clone = Arc::clone(&stop);

        ctrlc::set_handler(move || {
            stop_clone.store(true, Ordering::SeqCst);
        })
        .context("failed to set signal handler")?;

        if !config.quiet {
            output::recording(config.duration);
        }

        let mut cmd = Command::new("wf-recorder");

        if let Some(r) = region {
            cmd.args(["-g", &r.to_wf_recorder_arg()]);
        }

        cmd.args(["-r", &config.fps.to_string()])
            .args(["-c", "libx264rgb"])
            .args(["-p", "crf=18"])
            .args(["-f", output.to_str().unwrap()])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let mut child = cmd.spawn().context("failed to start wf-recorder")?;

        let poll = Duration::from_millis(50);
        let max_iters = if config.duration > 0.0 {
            ((config.duration * 1000.0) / poll.as_millis() as f32) as u64
        } else {
            u64::MAX
        };

        for _ in 0..max_iters {
            if stop.load(Ordering::SeqCst) {
                break;
            }
            if let Ok(Some(status)) = child.try_wait() {
                if !status.success() {
                    return Err(Error::Recording(format!("wf-recorder exited: {}", status)).into());
                }
                return Ok(());
            }
            std::thread::sleep(poll);
        }

        // Graceful shutdown
        #[allow(unsafe_code)]
        unsafe {
            libc::kill(child.id() as i32, libc::SIGINT);
        }

        let status = child.wait().context("failed to wait for wf-recorder")?;

        // SIGINT causes non-zero exit, which is expected
        if !status.success() && !stop.load(Ordering::SeqCst) && config.duration <= 0.0 {
            return Err(Error::Recording(format!("unexpected exit: {}", status)).into());
        }

        Ok(())
    }
}
