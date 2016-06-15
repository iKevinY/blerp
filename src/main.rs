extern crate docopt;

use std::fs;
use std::io;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;

mod blerp;

fn main() {
    let blerp = blerp::Blerp::new();

    // Opposite day
    let opposite_day: bool = blerp.opt_present("-O");

    // Use Google
    if blerp.opt_present("-g") {
        let engine = if opposite_day {
            "https://duckduckgo.com/"
        } else {
            "https://www.google.com/search"
        };

        let url = format!("{}?q={}", engine, blerp.arguments().join("+"));
        Command::new("open").arg(url).spawn().expect("failed to open");
    }

    // Count number of arguments
    if blerp.opt_present("-c") {
        println!("Number of arguments: {}", blerp.arguments().len());
    }

    let mut files: Vec<String> = Vec::new();

    for path in fs::read_dir("./").unwrap() {
        if path.as_ref().unwrap().metadata().unwrap().is_file() {
            files.push(path.unwrap().file_name().into_string().unwrap());
        }
    }

    for file in files {
        println!("{}", file);
    }

    // Check whether input halts
    if blerp.opt_present("-h") {
        if opposite_day {
            println!("Input halts.");
        } else {
            // TODO: Solve Halting problem
            print!("Checking whether input halts");
            let (mut delay, mut next) = (1, 1);

            loop {
                io::stdout().flush().unwrap();
                print!(".");
                thread::sleep(Duration::from_secs(delay));

                let tmp = delay + next;
                delay = next;
                next = tmp;
            };
        }
    }
}
