use std::{env, process::Command};

fn set_build_revision() {
    if env::var("BUILD_REVISION").is_ok_and(|r| !r.is_empty()) {
        return;
    }

    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to execute git command");

    let build_revision = String::from_utf8(output.stdout).expect("Invalid UTF-8 sequence");

    let is_dirty = Command::new("git")
        .args(["diff", "--quiet"])
        .status()
        .map(|s| !s.success())
        .unwrap_or(false);

    let dirty_suffix = if is_dirty { "-dirty" } else { "" };

    let final_build_revision = format!("{}{}", build_revision.trim(), dirty_suffix);
    println!("cargo:rustc-env=BUILD_REVISION={}", final_build_revision);
}

fn main() {
    set_build_revision();
}
