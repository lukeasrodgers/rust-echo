extern crate getopts;
extern crate echo;
use getopts::{optopt,optflag,getopts,OptGroup,usage};
use std::os;

use echo::{echo_newline, echo_no_newline, print_usage};

fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].clone();

    let opts = &[
        optflag("n", "no newline", "do not print trailing newline"),
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
        matches.free.clone()
    } else {
        print_usage(program.as_slice(), opts);
        return;
    };
    if matches.opt_present("n") {
        echo_no_newline(&input);
    }
    else {
        echo_newline(&input);
    }
}
