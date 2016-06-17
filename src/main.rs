extern crate ansi_term;
extern crate docopt;
extern crate rustc_serialize;

use std::fs;
use std::io;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;

use ansi_term::Style;

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

    // Stealth mode
    let mut style = Style::new();

    if blerp.flag_S {
        if blerp.flag_O {
            style = style.bold();
        } else {
            style = style.dimmed();
        }
    }

    // Determine if `say` or `espeak` are available
    let say_cmd: Option<&str>;

    if Command::new("say").spawn().is_ok() {
        say_cmd = Some("say");
    } else if Command::new("espeak").spawn().is_ok() {
        say_cmd = Some("espeak");
    } else {
        say_cmd = None;

        if blerp.flag_q && blerp.flag_O {
            println!("`say` and `espeak` are unavailable. Defaulting to quiet mode.")
        }
    }

    for mut file in files {
        // Suppress bees
        if blerp.flag_b {
            file = file.replace("B", "Ƀ").replace("b", "ƀ");
        }

        // Quiet mode, opposite day
        if blerp.flag_q && blerp.flag_O && say_cmd.is_some() {
            Command::new(String::from(say_cmd.unwrap())).arg(file).output().unwrap();
        } else {
            println!("{}", style.paint(file));
        }
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
