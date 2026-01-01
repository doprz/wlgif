use colored::Colorize;
use std::path::Path;

/// Print a status message (cyan arrow).
pub fn status(msg: &str) {
    eprintln!("{} {}", "→".cyan().bold(), msg);
}

/// Print a success message (green checkmark).
pub fn success(msg: &str) {
    eprintln!("{} {}", "✓".green().bold(), msg);
}

/// Print an info message (dim).
pub fn info(msg: &str) {
    eprintln!("  {}", msg.dimmed());
}

/// Print a warning (yellow).
#[allow(dead_code)]
pub fn warn(msg: &str) {
    eprintln!("{} {}", "⚠".yellow().bold(), msg);
}

/// Print recording status with duration info.
pub fn recording(duration: f32) {
    if duration > 0.0 {
        eprintln!(
            "{} Recording for {:.1}s {} {}",
            "●".red().bold(),
            duration,
            "-".dimmed(),
            "Ctrl+C to stop early".dimmed()
        );
    } else {
        eprintln!(
            "{} Recording {} {}",
            "●".red().bold(),
            "-".dimmed(),
            "Ctrl+C to stop".dimmed()
        );
    }
}

/// Print final summary after successful GIF creation.
pub fn summary(path: &Path, size_bytes: u64) {
    success(&format!("Saved: {}", path.display()));

    let size_kb = size_bytes as f64 / 1024.0;
    let size_str = if size_kb >= 1024.0 {
        format!("{:.2} MB", size_kb / 1024.0)
    } else {
        format!("{:.1} KB", size_kb)
    };
    info(&format!("Size: {}", size_str));
}
