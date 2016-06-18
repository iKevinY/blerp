use docopt::{ArgvMap, Docopt};

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
    pub arguments: Vec<String>,
    pub suppress_bees: bool,
    pub count_args: bool,
    pub deprecated: bool,
    pub fun_mode: bool,
    pub use_google: bool,
    pub check_if_halts: bool,
    pub opposite_day: bool,
    pub quiet_mode: bool,
    pub stealth_mode: bool,
}

impl Blerp {
    pub fn new() -> Self {
        let version = Some(format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
        let argvmap: ArgvMap = Docopt::new(USAGE)
                                      .unwrap_or_else(|e| e.exit())
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
}
