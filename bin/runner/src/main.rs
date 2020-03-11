#[macro_use]
extern crate lazy_static;
use console::Style;
use std::env;

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
    println!("{}", BOLD_CYAN.apply_to(title));
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        exit_and_warn();
    }
    let command_args = args.split_off(2);
    let command = args.get(1).unwrap();

    if command == BUILD_CMD {
        build_server::run_build_server(&command_args);
        build_client::run_build_client(&command_args);
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
