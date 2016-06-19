extern crate ansi_term;
extern crate docopt;
extern crate rustc_serialize;

use std::process;

mod blerp;

fn main() {
    let argv: Vec<String> = std::env::args().collect();

    match blerp::Blerp::new(argv).run() {
        Ok(_) => process::exit(0),
        Err(msg) => {
            println!("{}", msg);
            process::exit(1);
        }
    }
}
