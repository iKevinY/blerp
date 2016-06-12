use std::collections::HashSet;
use std::env;
use std::process;

const USAGE: &'static str = "
Usage:
    blerp [options] [args ...]
    blerp (--help | --version)

Options:
    -a    Attack mode
    -b    Suppress bees
    -—    Flags use em dashes
    -c    Count number of arguments
    -d    Pipes output to debug.exe
    -D    Deprecated
    -e    Execute something
    -f    Fun mode
    -g    Use Google
    -h    Check whether input halts
    -i    Ignore case (lower)
    -I    Ignore case (upper)
    -jk   Kidding
    -n    Behavior not defined
    -o    Overwrite
    -O    Opposite day
    -p    Set true Pope; accepts \"Rome\" or \"Avignon\"
    -q    Quiet mode; output is printed to stdout instead of being spoken aloud
    -r    Randomize arguments
    -R    Run recursively on http://*
    -s    Follow symbolic links symbolically
    -S    Stealth mode
    -t    Tumble dry
    -u    UTF-8 mode; otherwise defaults to ANSEL
    -U    Update (default: Facebook)
    -v    Verbose; alias to find / -exec cat {}
    -V    Set version number
    -y    Yikes
";

fn main() {
    // Roll a custom parser because the blerp specs are nonsense anyways.
    let mut args: Vec<String> = env::args().collect();
    let _program = args.remove(0);

    let mut flags: HashSet<char> = HashSet::new();
    let mut arguments: Vec<String> = Vec::new();
    let mut is_arg: bool = false;

    for arg in args {
        match arg.as_ref() {
            "--help" => {
                println!("{}", USAGE);
                process::exit(0);
            },

            "--version" => {
                println!("blerp {}", env!("CARGO_PKG_VERSION"));
                process::exit(0);
            },

            _ => {
                // Collect "short options" until an argument is encountered
                if arg.starts_with("--") || !arg.starts_with("-") || is_arg {
                    arguments.push(arg);
                    is_arg = true;
                } else {
                    for c in arg.chars() {
                        if c != '-' {
                            flags.insert(c);
                        }
                    }
                }
            },
        }
    }

    let opposite_day: bool = flags.contains(&'O');

    if flags.contains(&'g') {
        let engine = if opposite_day {
            "https://duckduckgo.com/"
        } else {
            "https://www.google.com/search"
        };

        let url = format!("{}?q={}", engine, arguments.join("+"));
        process::Command::new("open").arg(url).spawn().expect("failed to open");;
    }

    if flags.contains(&'h') {
        if opposite_day {
            println!("Input halts.");
        } else {
            // TODO: Solve Halting problem
            loop {};
        }
    }
}
