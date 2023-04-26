// mincd: Wait until a remote git repository has been changed.

mod argument_parser;
use argument_parser::Args;

extern crate mincd;
use mincd::wait_for_change;

use std::process::exit;

fn main() {
    let args = Args::new();
    match wait_for_change(args.url, args.branch, args.interval, args.local_path) {
        Ok(_) => exit(0),
        Err(e) => {
            println!("Error: {}", e);
            exit(1);
        }
    }
}
