extern crate getopts;
extern crate echo;
use getopts::{optopt,optflag,getopts,OptGroup,usage};
use std::os;

use echo::{echo_input, print_usage};

fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].clone();

    let opts = &[
        optopt("n", "", "do not print trailing newline", "NAME"),
        optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }
    let input = if !matches.free.is_empty() {
        matches.free
    } else {
        print_usage(program.as_slice(), opts);
        return;
    };
    echo_input(input);
}
