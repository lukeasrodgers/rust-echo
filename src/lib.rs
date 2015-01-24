extern crate getopts;

use getopts::{OptGroup,usage};
use std::os;

pub fn echo_newline(inp: &Vec<String>) {
    println!("{}", join(inp));
}

pub fn echo_no_newline(inp: &Vec<String>) {
    print!("{}", join(inp));
}

pub fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", usage(brief.as_slice(), opts));
}

fn join(vs: &Vec<String>) -> String {
    vs.connect(" ")
}

#[cfg(test)]
mod tests {
    use super::join;

    #[test]
    fn assert_join_single() {
        let s = "var".to_string();
        let v = vec![s.clone()];
        assert_eq!(join(&v), s);
    }

    #[test]
    fn assert_join_double() {
        let s = "var bar".to_string();
        let v = vec!["var".to_string(), "bar".to_string()];
        assert_eq!(join(&v), s);
    }
}
