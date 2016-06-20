extern crate ansi_term;
extern crate docopt;
extern crate rustc_serialize;

use std::fs;
use std::io;
use std::thread;

use std::io::Write;
use std::process::Command;
use std::time::Duration;

use ansi_term::Style;
use ansi_term::Colour::Red;
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

#[derive(Debug)]
pub struct Blerp {
    arguments: Vec<String>,
    suppress_bees: bool,
    count_args: bool,
    deprecated: bool,
    fun_mode: bool,
    use_google: bool,
    check_if_halts: bool,
    opposite_day: bool,
    quiet_mode: bool,
    stealth_mode: bool,
}

impl Blerp {
    pub fn new<S>(argv: Vec<S>) -> Self where S: AsRef<str> {
        let version = Some(format!("blerp {}", env!("CARGO_PKG_VERSION")));
        let argvmap = Docopt::new(USAGE)
                             .unwrap_or_else(|e| e.exit())
                             .argv(argv)
                             .options_first(true)
                             .version(version)
                             .parse()
                             .unwrap_or_else(|e| e.exit());

        Blerp {
            arguments:      argvmap.get_vec("<path>").iter().map(|a| a.to_string()).collect::<Vec<String>>(),
            suppress_bees:  argvmap.get_bool("-b"),
            count_args:     argvmap.get_bool("-c"),
            deprecated:     argvmap.get_bool("-D"),
            fun_mode:       argvmap.get_bool("-f"),
            use_google:     argvmap.get_bool("-g"),
            check_if_halts: argvmap.get_bool("-h"),
            opposite_day:   argvmap.get_bool("-O"),
            quiet_mode:     argvmap.get_bool("-q"),
            stealth_mode:   argvmap.get_bool("-S"),
        }
    }

    pub fn run(&self) -> Result<(), &str> {
        // Use Google
        if self.use_google {
            let engine = if self.opposite_day {
                "https://duckduckgo.com/"
            } else {
                "https://www.google.com/search"
            };

            let url = format!("{}?q={}", engine, self.arguments.join("+"));
            if Command::new("open").arg(url).output().is_err() {
                return Err("Failed to open URL in browser.");
            }
        }

        // Count number of arguments
        if self.count_args {
            println!("Number of arguments: {}", self.arguments.len());
        }

        // Deprecated feature
        if self.deprecated {
            println!("{}", Red.paint("Use of the `-D` option is deprecated."));
        }

        // Fun mode
        if self.fun_mode {
            println!("{}", Style::new().blink().bold().paint("Fun mode!"));
        }

        let mut files: Vec<String> = Vec::new();

        for path in fs::read_dir("./").unwrap() {
            if path.as_ref().unwrap().metadata().unwrap().is_file() {
                files.push(path.unwrap().file_name().into_string().unwrap());
            }
        }

        // Stealth mode
        let mut style = Style::new();

        if self.stealth_mode {
            if self.opposite_day {
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

            if self.quiet_mode && self.opposite_day {
                println!("`say` and `espeak` are unavailable. Defaulting to quiet mode.");
            }
        }

        for mut file in files {
            // Suppress bees
            if self.suppress_bees {
                file = file.replace("B", "Ƀ").replace("b", "ƀ");
            }

            // Quiet mode, opposite day
            if self.quiet_mode && self.opposite_day && say_cmd.is_some() {
                Command::new(String::from(say_cmd.unwrap())).arg(file).output().unwrap();
            } else {
                println!("{}", style.paint(file));
            }
        }

        // Check whether input halts
        if self.check_if_halts {
            if self.opposite_day {
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
                }
            }
        }

        return Ok(());
    }
}
