use std::process::Command;

#[test]
fn test_initial_message() {
    let output = Command::new("cargo")
        .arg("run")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert_eq!(stdout.trim(), "Starting Todo App");
}
