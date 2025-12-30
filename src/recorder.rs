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

/// Record a screen region to a video file.
pub fn record(region: &Region, output: &Path, fps: u32, duration: f32, quiet: bool) -> Result<()> {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = Arc::clone(&stop);

    ctrlc::set_handler(move || {
        stop_clone.store(true, Ordering::SeqCst);
    })
    .context("failed to set signal handler")?;

    if !quiet {
        output::recording(duration);
    }

    let mut child = Command::new("wf-recorder")
        .args(["-g", &region.to_wf_recorder_arg()])
        .args(["-r", &fps.to_string()])
        .args(["-c", "libx264rgb"])
        .args(["-p", "crf=18"])
        .args(["-f", output.to_str().unwrap()])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("failed to start wf-recorder")?;

    let poll = Duration::from_millis(50);
    let max_iters = if duration > 0.0 {
        ((duration * 1000.0) / poll.as_millis() as f32) as u64
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
    if !status.success() && !stop.load(Ordering::SeqCst) && duration <= 0.0 {
        return Err(Error::Recording(format!("unexpected exit: {}", status)).into());
    }

    Ok(())
}
