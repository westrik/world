#[macro_use]
extern crate lazy_static;
use console::Style;
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
use std::time::Instant;
use std::{env, thread};

mod build_client;
mod build_server;

const BUILD_CMD: &str = "build";
const TEST_CMD: &str = "test";
const RUN_SERVER_CMD: &str = "run_server";
const RUN_CLIENT_CMD: &str = "run_client";
const FORMAT_CMD: &str = "format";

lazy_static! {
    static ref BOLD_CYAN: Style = Style::new().bold().cyan();
    static ref BOLD_RED: Style = Style::new().bold().red();
}

fn exit_and_warn() {
    eprintln!("{}", BOLD_RED.apply_to("invalid arguments!"));
    eprintln!(
        "usage: bin/dev [{}|{}|{}|{}]",
        BUILD_CMD, TEST_CMD, RUN_SERVER_CMD, RUN_CLIENT_CMD
    );
    std::process::exit(1);
}

fn print_title(title: &str) {
    print!("{}", BOLD_CYAN.apply_to(title));
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        exit_and_warn();
    }
    let command_args = args.split_off(3);
    let path = args.get(1).unwrap();
    let command = args.get(2).unwrap();

    if command == BUILD_CMD {
        let mut thread_handles = vec![];
        let m = MultiProgress::new();
        let started = Instant::now();

        let pb = m.add(ProgressBar::new_spinner());
        let project_path = path.to_string();
        let build_args = command_args.clone();
        let _thread = thread::spawn(move || {
            pb.enable_steady_tick(30);
            pb.set_message("running server build");
            build_server::run_build_server(&project_path, &build_args);
            pb.set_message(
                format!(
                    "server build completed in {}",
                    HumanDuration(started.elapsed())
                )
                .as_str(),
            );
            pb.finish();
        });
        thread_handles.push(_thread);

        let pb = m.add(ProgressBar::new_spinner());
        let project_path = path.to_string();
        let build_args = command_args.clone();
        let _thread = thread::spawn(move || {
            pb.enable_steady_tick(30);
            pb.set_message("running client build");
            build_client::run_build_client(&project_path, &build_args);
            pb.set_message(
                format!(
                    "client build completed in {}",
                    HumanDuration(started.elapsed())
                )
                .as_str(),
            );
            pb.finish();
        });
        thread_handles.push(_thread);
        m.join().unwrap();
    } else if command == TEST_CMD {
        unimplemented!();
    } else if command == RUN_CLIENT_CMD {
        unimplemented!();
    } else if command == RUN_SERVER_CMD {
        unimplemented!();
    } else if command == FORMAT_CMD {
        unimplemented!();
    } else {
        exit_and_warn();
    }
}
