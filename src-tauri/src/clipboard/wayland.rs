use super::ClipboardWatcher;
use anyhow::{Result, anyhow};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub struct WaylandWatcher;

impl ClipboardWatcher for WaylandWatcher {
    fn watch(&self, on_change: Box<dyn Fn(String) + Send + Sync>) -> Result<()> {
        let mut child = Command::new("wl-paste")
            .arg("--watch")
            .arg("wl-paste")
            .arg("-n")
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|_| anyhow!("wl-clipboard not installed?"))?;

        let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed to open stdout"))?;
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            if let Ok(content) = line {
                if !content.trim().is_empty() {
                    on_change(content);
                }
            }
        }
        Ok(())
    }
}