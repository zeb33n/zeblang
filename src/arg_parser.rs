use std::collections::HashMap;
use std::env;

pub fn parse_args<'a>() -> HashMap<&'a str, String> {
    //let args: Vec<String> = env::args().collect();
    let mut out: HashMap<&str, String> = HashMap::new();
    for arg in env::args().into_iter() {
        match arg.as_str() {
            "-j" | "--json" => out.insert("json", arg),
            filename if filename.ends_with(".zb") => out.insert("filename", arg),
            _ => continue,
        };
    }
    out
}
