extern crate blerp;
use blerp::Blerp;

use std::env;
use std::process;

fn main() {
    let argv: Vec<String> = env::args().collect();

    match Blerp::new(argv) {
        Ok(blerp) => {
            if let Err(msg) = blerp.run() {
                println!("{}", msg);
                process::exit(1);
            }
        }
        Err(err) => {
            println!("{}", err);
            if err.fatal() {
                process::exit(1);
            }
        }
    }
}
