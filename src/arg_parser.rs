use std::collections::HashMap;
use std::env;

pub enum TargetKind {
    Json,
    Llvm,
}

pub enum Arg {
    Filename(String),
    Target(TargetKind),
}

pub fn parse_args() -> HashMap<&'static str, Arg> {
    let mut out: HashMap<&str, Arg> = HashMap::new();
    for arg in env::args().into_iter() {
        println!("{}", &arg);
        match arg.as_str() {
            "-j" | "--json" => out.insert("target", Arg::Target(TargetKind::Json)),
            "-l" | "--llvm" => out.insert("target", Arg::Target(TargetKind::Llvm)),
            filename if filename.ends_with(".zb") => out.insert("filename", Arg::Filename(arg)),
            _ => continue,
        };
    }
    out
}
