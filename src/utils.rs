use std::fs::OpenOptions;
use std::io::{Write, Result};
use std::process::{Command, Stdio};

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    if std::env::var("MOCK_CLIPBOARD").is_ok() {
        println!("{} passwords successfully copied to clipboard", text.lines().count());
        return Ok(());
    }

    #[cfg(target_os = "linux")]
    let mut child = Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(Stdio::piped())
        .spawn()?;

    #[cfg(target_os = "macos")]
    let mut child = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()?;

    #[cfg(target_os = "windows")]
    let mut child = Command::new("cmd")
        .args(["/C", "clip"])
        .stdin(Stdio::piped())
        .spawn()?;

    child.stdin.as_mut().unwrap().write_all(text.as_bytes())?;

    println!("{} passwords successfully copied to clipboard", text.lines().count());
    Ok(())
}

pub fn save_to_file(path: &str, text: &str) -> Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{}", text)?;
    Ok(())
}

pub fn calculate_entropy(length: usize, charset_size: f64) -> f64 {
    let log2 = charset_size.log2();
    (length as f64) * log2
}
