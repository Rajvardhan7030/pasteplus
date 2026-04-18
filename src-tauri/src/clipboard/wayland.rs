use super::ClipboardWatcher;
use anyhow::{Result, anyhow};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub struct WaylandWatcher;

impl ClipboardWatcher for WaylandWatcher {
    fn watch(&self, on_change: Box<dyn Fn(String) + Send + Sync>) -> Result<()> {
        let mut child = Command::new("wl-paste")
            .arg("--watch")
            .arg("sh")
            .arg("-c")
            .arg("echo _CHANGE_")
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|_| anyhow!("wl-clipboard not installed?"))?;

        let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed to open stdout"))?;
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            if let Ok(_) = line {
                let output = Command::new("wl-paste")
                    .arg("-n")
                    .output();
                
                if let Ok(out) = output {
                    let content = String::from_utf8_lossy(&out.stdout).to_string();
                    if !content.trim().is_empty() {
                        on_change(content);
                    }
                }
            }
        }
        Ok(())
    }

    fn set_content(&self, content: String) -> Result<()> {
        use std::io::Write;
        let mut child = Command::new("wl-copy")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|_| anyhow!("wl-copy not installed?"))?;

        let mut stdin = child.stdin.take().ok_or_else(|| anyhow!("Failed to open stdin"))?;
        stdin.write_all(content.as_bytes())?;
        drop(stdin);
        child.wait()?;
        Ok(())
    }
}