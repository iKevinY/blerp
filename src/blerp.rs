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

#[allow(non_snake_case)]
#[derive(Debug, RustcDecodable)]
pub struct Blerp {
    pub arg_path: Vec<String>,
    pub flag_b: bool,
    pub flag_c: bool,
    pub flag_g: bool,
    pub flag_h: bool,
    pub flag_O: bool,
    pub flag_q: bool,
    pub flag_S: bool,
}

impl Blerp {
    pub fn new() -> Self {
        let version = Some(format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
        Docopt::new(USAGE)
               .unwrap_or_else(|e| e.exit())
               .options_first(true)
               .version(version)
               .decode()
               .unwrap_or_else(|e| e.exit())
    }
}
