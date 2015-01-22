extern crate regex;
#[plugin] #[no_link] extern crate regex_macros;

pub fn interpolate(s: String) -> String {
    s
}

fn get_possible_env_vars(s: &String) -> Option<regex::Captures> {
    let re = regex!(r"\$[^\s]+");
    re.captures(s.as_slice())
}

#[cfg(test)]
mod tests {
    use super::interpolate;

    fn assert_interpolate_home() {
        // assumes, which is probably fine for now, that HOME is set
        let s = "var$HOME".to_string();
        assert_eq!(interpolate(s), "var/Users/luke".to_string());
    }
}
