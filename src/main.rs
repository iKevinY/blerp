extern crate blerp;
use blerp::Blerp;

use std::env;
use std::process;

fn main() {
    let argv: Vec<String> = env::args().collect();

    match Blerp::new(argv).run() {
        Ok(_) => process::exit(0),
        Err(msg) => {
            println!("{}", msg);
            process::exit(1);
        }
    }
}
