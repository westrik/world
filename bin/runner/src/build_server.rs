use crate::print_title;
use std::process::{Command, Stdio};

pub fn run_build_server(path: &str, _args: &Vec<String>) {
    // print_title("building server...");
    let path = format!("{}/server", path);

    let _result = Command::new("bash")
        .args(&["-c", "cargo build"])
        .current_dir(path.as_str())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output();
}
