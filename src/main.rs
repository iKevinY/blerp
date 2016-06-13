extern crate docopt;

use std::process::Command;

use docopt::Docopt;

const USAGE: &'static str = "
blerp filters local or remote files or resources.

Usage:
    blerp [options] [<path>...]
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
    let args = Docopt::new(USAGE).and_then(|dopt| dopt.parse()).unwrap_or_else(|e| e.exit());

    if args.get_bool("--version") {
        println!("blerp {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    // Opposite day
    let opposite_day: bool = args.get_bool("-O");

    // Use Google
    if args.get_bool("-g") {
        let engine = if opposite_day {
            "https://duckduckgo.com/"
        } else {
            "https://www.google.com/search"
        };

        let url = format!("{}?q={}", engine, args.get_vec("<path>").join("+"));
        Command::new("open").arg(url).spawn().expect("failed to open");
    }

    // Count number of arguments
    if args.get_bool("-c") {
        println!("Number of arguments: {}", args.get_vec("<path>").len());
    }

    // Check whether input halts
    if args.get_bool("-h") {
        if opposite_day {
            println!("Input halts.");
        } else {
            // TODO: Solve Halting problem
            loop {};
        }
    }
}
