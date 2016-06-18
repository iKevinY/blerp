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
use ansi_term::Colour::Red;

mod blerp;

fn main() {
    let blerp = blerp::Blerp::new();

    // Use Google
    if blerp.use_google() {
        let engine = if blerp.opposite_day() {
            "https://duckduckgo.com/"
        } else {
            "https://www.google.com/search"
        };

        let url = format!("{}?q={}", engine, blerp.arguments().join("+"));
        Command::new("open").arg(url).spawn().expect("failed to open");
    }

    // Count number of arguments
    if blerp.count_args() {
        println!("Number of arguments: {}", blerp.arguments().len());
    }

    // Deprecated feature
    if blerp.deprecated() {
        println!("{}", Red.paint("Use of the `-D` option is deprecated."));
    }

    let mut files: Vec<String> = Vec::new();

    for path in fs::read_dir("./").unwrap() {
        if path.as_ref().unwrap().metadata().unwrap().is_file() {
            files.push(path.unwrap().file_name().into_string().unwrap());
        }
    }

    // Stealth mode
    let mut style = Style::new();

    if blerp.stealth_mode() {
        if blerp.opposite_day() {
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

        if blerp.quiet_mode() && blerp.opposite_day() {
            println!("`say` and `espeak` are unavailable. Defaulting to quiet mode.")
        }
    }

    for mut file in files {
        // Suppress bees
        if blerp.suppress_bees() {
            file = file.replace("B", "Ƀ").replace("b", "ƀ");
        }

        // Quiet mode, opposite day
        if blerp.quiet_mode() && blerp.opposite_day() && say_cmd.is_some() {
            Command::new(String::from(say_cmd.unwrap())).arg(file).output().unwrap();
        } else {
            println!("{}", style.paint(file));
        }
    }

    // Check whether input halts
    if blerp.check_if_halts() {
        if blerp.opposite_day() {
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
