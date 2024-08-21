use std::process::Command;

#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("version")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.trim().starts_with("gen_todo version"));
}
