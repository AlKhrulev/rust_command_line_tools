#![allow(dead_code)]
use std::env::args;
use std::path::Path;
use std::process::exit;
use std::process::Command;
use std::time::{Duration, Instant};
// NOTE don't forget to strip the debugging symbols

// NOTE use output() instead of status() to avoid stdout piping
fn time_file_once(expression: &str) -> Duration {
    dbg!("time_file_once");
    let now = Instant::now();
    let _status = Command::new(expression)
        .spawn()
        .expect("failed to spawn a process")
        .wait() // helps to release the allocated resources. BLOCKING!
        .expect("failed to wait on child");
    // dbg!("spawned child's status is ",_status);
    now.elapsed()
}

fn time_command_once(command: &str, args: &[String]) -> Duration {
    dbg!("time_command_once");
    let now = Instant::now();
    let _status = Command::new(command)
        .args(args)
        .status()
        .expect("failed to execute process");
    // dbg!("spawned child's status is ",status);
    now.elapsed()
}

fn main() {
    for argument in args() {
        println!("{argument}");
    }
    let mut args_iter = args().skip(1); //skip executable name
    let first_argument: String = args_iter
        .next()
        .expect("First argument to the executable is not provided");

    if first_argument.chars().count() == 0 {
        println!("Can't have an empty second argument!");
        exit(1);
    }
    if first_argument.to_lowercase()=="-h" {
        println!("USAGE: timeit FILENAME or timeit COMMAND_TO_EXECUTE");
        exit(0);
    }
    let possible_path = Path::new(&first_argument);
    if possible_path.exists() {
        dbg!("Executing the file at ", possible_path);
        time_file_once(&first_argument);
    } else {
        let command_arguments: Vec<String> = args_iter.collect();
        dbg!("Executing the command");
        time_command_once(&first_argument, command_arguments.as_slice());
        let time=time_command_once(&first_argument, command_arguments.as_slice()).as_nanos();
        println!("{:} `\u{33B1}",time);
    }
}
