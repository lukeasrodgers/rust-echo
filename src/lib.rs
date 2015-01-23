extern crate getopts;

use getopts::{OptGroup,usage};
use std::os;

pub fn echo_newline(inp: &Vec<String>) {
    let massaged = massage(inp);
    println!("{}", massaged);
}

pub fn echo_no_newline(inp: &Vec<String>) {
    let massaged = massage(inp);
    print!("{}", massaged);
}

fn massage(vs: &Vec<String>) -> String {
    join(vs)
}

pub fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", usage(brief.as_slice(), opts));
}

fn join(vs: &Vec<String>) -> String {
    let mut result = "".to_string();
    for s in vs.iter() {
        result.push_str(s.as_slice());
        result.push_str(" ");
    }
    // remove trailing space
    result.pop();
    result
}

#[cfg(test)]
mod tests {
    use super::massage;

    #[test]
    fn assert_massage_single() {
        let s = "var".to_string();
        let v = vec![s.clone()];
        assert_eq!(massage(&v), s);
    }

    #[test]
    fn assert_massage_double() {
        let s = "var bar".to_string();
        let v = vec!["var".to_string(), "bar".to_string()];
        assert_eq!(massage(&v), s);
    }
}
