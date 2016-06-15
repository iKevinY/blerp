extern crate docopt;

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

pub struct Blerp {
    argvmap: ArgvMap,
}

impl Blerp {
    pub fn new() -> Self {
        Blerp {
            argvmap: Docopt::new(USAGE).and_then(|d| d.parse()).unwrap_or_else(|e| e.exit()),
        }
    }

    pub fn opt_present(&self, f: &str) -> bool {
        self.argvmap.get_bool(f)
    }

    pub fn arguments(&self) -> Vec<&str> {
        self.argvmap.get_vec("<path>")
    }
}
