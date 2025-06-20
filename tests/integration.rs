use std::process::Command;
use std::time::Instant;


fn run_cli(args: &[&str]) -> String {
    run_cli_with_env(args, &[])
}

fn run_cli_with_env(args: &[&str], envs: &[(&str, &str)]) -> String {
    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--quiet", "--"]).args(args);

    for (key, val) in envs {
        cmd.env(key, val);
    }

    let output = cmd.output().expect("Failed to run cargo");

    if !output.status.success() {
        panic!("Process failed:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn default_password() {
    let stdout = run_cli(&["--length", "12"]);
    assert!(stdout.contains("Password:"));
    let pwd = stdout.trim().split(": ").nth(1).unwrap();
    assert_eq!(pwd.len(), 12);
}

#[test]
fn alpha_only() {
    let stdout = run_cli(&["-a", "-l", "10"]);
    assert!(stdout.contains("Password:"));
    let pwd = stdout.trim().split(": ").nth(1).unwrap();
    assert!(pwd.chars().all(|c| c.is_ascii_alphabetic()));
    assert_eq!(pwd.len(), 10);
}

#[test]
fn numeric_only() {
    let stdout = run_cli(&["-n", "-l", "8"]);
    assert!(stdout.contains("Password:"));
    let pwd = stdout.trim().split(": ").nth(1).unwrap();
    assert!(pwd.chars().all(|c| c.is_ascii_digit()));
    assert_eq!(pwd.len(), 8);
}

#[test]
fn alphanumeric_no_symbols() {
    let stdout = run_cli(&["-an", "-l", "15"]);
    assert!(stdout.contains("Password:"));
    let pwd = stdout.trim().split(": ").nth(1).unwrap();
    assert!(pwd.chars().all(|c| c.is_ascii_alphanumeric()));
    assert_eq!(pwd.len(), 15);
}

#[test]
fn no_repeat_flag() {
    let stdout = run_cli(&["--no-repeat", "-l", "20"]);
    assert!(stdout.contains("Password:"));
    let pwd = stdout.trim().split(": ").nth(1).unwrap();
    let mut chars = pwd.chars().collect::<Vec<_>>();
    chars.sort_unstable();
    chars.dedup();
    assert_eq!(chars.len(), pwd.len());
}

#[test]
fn exclude_flag() {
    let stdout = run_cli(&["--exclude", "abc123", "-l", "20"]);
    assert!(stdout.contains("Password:"));
    let pwd = stdout.trim().split(": ").nth(1).unwrap();
    for c in "abc123".chars() {
        assert!(!pwd.contains(c));
    }
}

#[test]
fn count_multiple_passwords() {
    let stdout = run_cli(&["-c", "3", "-l", "10"]);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 3);
    for line in lines {
        assert!(line.starts_with("Password: "));
        let pwd = line.split(": ").nth(1).unwrap();
        assert_eq!(pwd.len(), 10);
    }
}

#[test]
fn invalid_flag_fails() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "--invalidflag"])
        .output()
        .expect("Failed to run cargo");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid flag"));
}

#[test]
fn clipboard_flag() {
    let stdout = run_cli_with_env(&["--clipboard", "-l", "12"], &[("MOCK_CLIPBOARD", "1")]);
    assert!(stdout.contains("Password successfully copied to clipboard"));
    assert!(!stdout.contains("Password:"));
}

#[test]
fn multiple_clipboard() {
    let stdout = run_cli(&["-l", "5", "-c", "3"]);
    let lines: Vec<&str> = stdout.lines().filter(|l| l.starts_with("Password:")).collect();
    assert_eq!(lines.len(), 3);
}

#[test]
fn entropy_flag_prints_entropy() {
    let stdout = run_cli(&["--entropy", "-l", "12"]);
    assert!(stdout.contains("Estimated entropy"));
}

#[test]
fn alpha_no_repeat_exclude() {
    let stdout = run_cli(&["-a", "--no-repeat", "-ex", "aeiouAEIOU", "-l", "15"]);
    assert!(stdout.contains("Password:"));
    let pwd = stdout.trim().split(": ").nth(1).unwrap();
    assert_eq!(pwd.len(), 15);
    assert!(pwd.chars().all(|c| c.is_ascii_alphabetic()));
    for c in "aeiouAEIOU".chars() {
        assert!(!pwd.contains(c));
    }
    let mut chars: Vec<char> = pwd.chars().collect();
    chars.sort_unstable();
    chars.dedup();
    assert_eq!(chars.len(), pwd.len());
}

#[test]
fn numeric_count_clipboard() {
    let stdout = run_cli_with_env(&["-n", "-c", "3", "-cp", "-l", "8"], &[("MOCK_CLIPBOARD", "1")]);
    assert!(stdout.contains("3 passwords successfully copied to clipboard"));
    let lines: Vec<&str> = stdout.lines().filter(|l| l.starts_with("Password:")).collect();
    assert_eq!(lines.len(), 0);
}

#[test]
fn alphanumeric_save_entropy() {
    let path = "test_output.txt";
    let stdout = run_cli(&["-an", "-s", path, "-en", "-l", "12"]);
    assert!(stdout.contains("Estimated entropy"));
    assert!(stdout.contains(path));
    assert!(!stdout.contains("Password:"));
    let _ = std::fs::remove_file(path);
}

#[test]
fn all_flags_combined() {
    let path = "test_all_flags.txt";
    let stdout = run_cli_with_env(
        &[
            "-a", "-nr", "-c", "2", "-ex", "0OIl1", "-cp", "-s", path, "-en", "-l", "10",
        ],
        &[("MOCK_CLIPBOARD", "1")],
    );
    assert!(stdout.contains("2 passwords successfully copied to clipboard"));
    assert!(stdout.contains(&format!("2 passwords written to file at {}", path)));
    assert!(stdout.contains("Estimated entropy"));
    let lines: Vec<&str> = stdout.lines().filter(|l| l.starts_with("Password:")).collect();
    assert_eq!(lines.len(), 0);
    let _ = std::fs::remove_file(path);
}

#[test]
fn performance_large_password_generation() {
    let count = "100000";
    let length = "12";
    let args = ["-c", count, "-l", length];

    let start = Instant::now();
    let stdout = run_cli(&args);
    let duration = start.elapsed();

    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 10_0000);

    for line in &lines {
        assert!(line.starts_with("Password: "));
        let pwd = line.split(": ").nth(1).unwrap();
        assert_eq!(pwd.len(), 12);
    }

    assert!(
        duration.as_secs_f32() < 10.0,
        "Generation took too long: {:.2} seconds",
        duration.as_secs_f32()
    );

}
