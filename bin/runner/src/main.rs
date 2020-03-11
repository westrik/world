#[macro_use]
extern crate lazy_static;
use console::Style;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use std::time::Instant;
use std::{env, thread};

mod build_client;
mod build_server;

// TODO: configure with TOML

const BUILD_CMD: &str = "build";
const CHECK_CMD: &str = "check";
const RUN_SERVERS_CMD: &str = "run";

lazy_static! {
    static ref BOLD_CYAN: Style = Style::new().bold().cyan();
    static ref RED: Style = Style::new().red();
}

fn exit_and_warn() {
    eprintln!("{}", RED.apply_to("invalid arguments!"));
    eprintln!(
        "usage: bin/dev [{}|{}|{}]",
        BUILD_CMD, CHECK_CMD, RUN_SERVERS_CMD
    );
    std::process::exit(1);
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

        // task definition:
        // - name ("building server")
        // - subdirectory (server/)
        // - command ("cargo build --release")
        // - stdout location (None=/dev/null)
        // - stderr location (None=/dev/null)

        // TODO: refactor
        let style = ProgressStyle::default_spinner().tick_chars("✶✸✹✺✹✷ ");
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            style
                .clone()
                .template(format!("☁️  {{msg}} {}", BOLD_CYAN.apply_to("{spinner}")).as_str()),
        );
        let pb = m.add(spinner);
        let project_path = path.to_string();
        let build_args = command_args.clone();
        let _thread = thread::spawn(move || {
            pb.enable_steady_tick(66);
            pb.set_message("building server");
            build_server::run_build_server(&project_path, &build_args);
            pb.set_message(
                format!("building server took {}", HumanDuration(started.elapsed())).as_str(),
            );
            pb.finish();
        });
        thread_handles.push(_thread);

        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            style
                .clone()
                .template(format!("💻 {{msg}} {}", BOLD_CYAN.apply_to("{spinner}")).as_str()),
        );
        let pb = m.add(spinner);
        let project_path = path.to_string();
        let build_args = command_args.clone();
        let _thread = thread::spawn(move || {
            pb.enable_steady_tick(66);
            pb.set_message("building client");
            build_client::run_build_client(&project_path, &build_args);
            pb.set_message(
                format!("building client took {}", HumanDuration(started.elapsed())).as_str(),
            );
            pb.finish();
        });
        thread_handles.push(_thread);
        m.join().unwrap();
    } else if command == CHECK_CMD {
        unimplemented!();
    } else if command == RUN_SERVERS_CMD {
        // https://docs.rs/notify/5.0.0-pre.2/notify/index.html
        // TODO: on file change: run check task, rebuild, restart server
        unimplemented!();
    } else {
        exit_and_warn();
    }
}
