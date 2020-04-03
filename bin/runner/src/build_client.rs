use std::process::{Command, Stdio};

pub fn run_build_client(path: &str, _args: &Vec<String>) {
    // print_title("building client...");
    let path = format!("{}/client-web/", path);

    let _result = Command::new("bash")
        .args(&["-c", "yarn && yarn build"])
        .current_dir(path.as_str())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output();
}
