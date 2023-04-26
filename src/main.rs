// mincd: Wait until a remote git repository has been changed.

mod argument_parser;
use argument_parser::Args;

extern crate mincd;
use mincd::wait_for_change;

fn main() {
    let args = Args::new();
    wait_for_change(args.url, args.branch, args.interval, args.local_path);
}
