extern crate docopt;
extern crate rustc_serialize;

use std::fs;
use std::io;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;

mod blerp;

fn main() {
    let blerp = blerp::Blerp::new();

    // Use Google
    if blerp.flag_g {
        let engine = if blerp.flag_O {
            "https://duckduckgo.com/"
        } else {
            "https://www.google.com/search"
        };

        let url = format!("{}?q={}", engine, blerp.arg_path.join("+"));
        Command::new("open").arg(url).spawn().expect("failed to open");
    }

    // Count number of arguments
    if blerp.flag_c {
        println!("Number of arguments: {}", blerp.arg_path.len());
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
    if blerp.flag_h {
        if blerp.flag_O {
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
