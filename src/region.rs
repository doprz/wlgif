use crate::error::Error;
use crate::output;
use anyhow::{Context, Result};
use std::process::{Command, Stdio};

/// A rectangular screen region.
#[derive(Debug, Clone, Copy)]
pub struct Region {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Region {
    /// Parse from slurp output format: "X,Y WxH"
    pub fn from_slurp(s: &str) -> Result<Self> {
        let s = s.trim();
        let mut parts = s.split_whitespace();

        let pos = parts.next().context("missing position in slurp output")?;
        let size = parts.next().context("missing size in slurp output")?;

        let mut pos_parts = pos.split(',');
        let x: u32 = pos_parts
            .next()
            .context("missing X")?
            .parse()
            .context("invalid X coordinate")?;
        let y: u32 = pos_parts
            .next()
            .context("missing Y")?
            .parse()
            .context("invalid Y coordinate")?;

        let mut size_parts = size.split('x');
        let width: u32 = size_parts
            .next()
            .context("missing width")?
            .parse()
            .context("invalid width")?;
        let height: u32 = size_parts
            .next()
            .context("missing height")?
            .parse()
            .context("invalid height")?;

        Ok(Self {
            x,
            y,
            width,
            height,
        })
    }

    /// Parse from geometry string: "WxH+X+Y"
    pub fn from_geometry(s: &str) -> Result<Self> {
        let s = s.trim();
        let parts: Vec<&str> = s.split(&['x', '+'][..]).collect();

        if parts.len() != 4 {
            return Err(Error::InvalidRegion(s.to_string()).into());
        }

        let width: u32 = parts[0]
            .parse()
            .map_err(|_| Error::InvalidRegion(s.to_string()))?;
        let height: u32 = parts[1]
            .parse()
            .map_err(|_| Error::InvalidRegion(s.to_string()))?;
        let x: u32 = parts[2]
            .parse()
            .map_err(|_| Error::InvalidRegion(s.to_string()))?;
        let y: u32 = parts[3]
            .parse()
            .map_err(|_| Error::InvalidRegion(s.to_string()))?;

        Ok(Self {
            x,
            y,
            width,
            height,
        })
    }

    /// Format for wf-recorder's -g flag: "X,Y WxH"
    pub fn to_wf_recorder_arg(self) -> String {
        format!("{},{} {}x{}", self.x, self.y, self.width, self.height)
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}x{} at ({}, {})",
            self.width, self.height, self.x, self.y
        )
    }
}

/// Interactive region selection using slurp.
pub fn select_interactive(quiet: bool) -> Result<Region> {
    if !quiet {
        output::status("Select a region...");
    }

    let result = Command::new("slurp")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .context("failed to execute slurp")?;

    if !result.status.success() {
        return Err(Error::SelectionCancelled.into());
    }

    let stdout = String::from_utf8_lossy(&result.stdout);
    Region::from_slurp(&stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_slurp_output() {
        let region = Region::from_slurp("100,200 800x600").unwrap();
        assert_eq!(region.x, 100);
        assert_eq!(region.y, 200);
        assert_eq!(region.width, 800);
        assert_eq!(region.height, 600);
    }

    #[test]
    fn parse_geometry() {
        let region = Region::from_geometry("800x600+100+200").unwrap();
        assert_eq!(region.width, 800);
        assert_eq!(region.height, 600);
        assert_eq!(region.x, 100);
        assert_eq!(region.y, 200);
    }

    #[test]
    fn wf_recorder_format() {
        let region = Region {
            x: 100,
            y: 200,
            width: 800,
            height: 600,
        };
        assert_eq!(region.to_wf_recorder_arg(), "100,200 800x600");
    }
}
