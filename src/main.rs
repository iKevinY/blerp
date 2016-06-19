extern crate ansi_term;
extern crate docopt;
extern crate rustc_serialize;

use std::process;

mod blerp;

fn main() {
    let argv: Vec<String> = std::env::args().collect();

    match blerp::Blerp::new(argv) {
        Ok(blerp) => blerp.run(),
        Err(msg) => {
            println!("{}", msg);
            process::exit(1);
        }
    }
}
