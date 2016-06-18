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
    arg_path: Vec<String>,
    flag_b: bool,
    flag_c: bool,
    flag_g: bool,
    flag_h: bool,
    flag_O: bool,
    flag_q: bool,
    flag_S: bool,
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

    // Getter methods for struct fields
    pub fn arguments(&self) -> &Vec<String> { &self.arg_path }

    pub fn suppress_bees(&self)     -> bool { self.flag_b }
    pub fn count_args(&self)        -> bool { self.flag_c }
    pub fn use_google(&self)        -> bool { self.flag_g }
    pub fn check_if_halts(&self)    -> bool { self.flag_h }
    pub fn opposite_day(&self)      -> bool { self.flag_O }
    pub fn quiet_mode(&self)        -> bool { self.flag_q }
    pub fn stealth_mode(&self)      -> bool { self.flag_S }
}
