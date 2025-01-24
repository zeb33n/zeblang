use std::collections::HashMap;
use std::env;

pub fn parse_args() -> HashMap<&'static str, String> {
    let mut out: HashMap<&str, String> = HashMap::new();
    for arg in env::args().into_iter() {
        match arg.as_str() {
            "-j" | "--json" => out.insert("json", arg),
            "-i" | "--interpret" => out.insert("interpret", arg),
            filename if filename.ends_with(".zb") => out.insert("filename", arg),
            _ => continue,
        };
    }
    out
}
