use std::fs::read_to_string;
use std::process::Command;

fn extract_value_from_stdout(stdout: &Vec<u8>) -> u8 {
    String::from_utf8_lossy(stdout)
        .replace("\n", "")
        .rsplit_once(" ")
        .unwrap()
        .1
        .parse()
        .unwrap()
}

fn run_zeblang_file(addr: &str) -> u8 {
    let output = Command::new("bash")
        .arg("test_scripts/test_compile.sh")
        .arg(addr)
        .output()
        .expect("failed");
    extract_value_from_stdout(&output.stdout)
}

fn run_zeblang_file_json(addr: &str) -> String {
    let _ = Command::new("bash")
        .arg("test_scripts/test_json.sh")
        .arg(addr)
        .output();
    let out = read_to_string(addr.replace(".zb", ".json")).expect("failed");
    let _ = Command::new("rm")
        .arg(addr.replace(".zb", ".json"))
        .output();
    out
}

#[test]
fn test_for() {
    let out = run_zeblang_file("test_scripts/for.zb");
    assert_eq!(8, out);
}

#[test]
fn test_if() {
    let out = run_zeblang_file("test_scripts/if.zb");
    assert_eq!(3, out);
}

#[test]
fn test_arrays() {
    let out = run_zeblang_file("test_scripts/arrays.zb");
    assert_eq!(10, out);
}

#[test]
fn test_mut_arrays() {
    let out = run_zeblang_file("test_scripts/mut_arrays.zb");
    assert_eq!(8, out);
}

#[test]
fn test_while() {
    let out = run_zeblang_file("test_scripts/while.zb");
    assert_eq!(12, out);
}

#[test]
fn test_precedance() {
    let out = run_zeblang_file("test_scripts/precedance.zb");
    assert_eq!(13, out);
}

#[test]
fn test_variables() {
    let out = run_zeblang_file("test_scripts/variables.zb");
    assert_eq!(3, out);
}

#[test]
fn test_json_syntax_error() {
    let out = run_zeblang_file_json("test_scripts/syntax_error.zb");
    assert_eq!("\"2: expected operator\"".to_string(), out);
}

#[test]
fn test_json() {
    let out = run_zeblang_file_json("test_scripts/json.zb");
    assert_eq!(
        "[\n  {\n    \"Assign\": [\n      \"var\",\n      {\n        \"Array\": [\n          {\n            \"Value\": \"1\"\n          },\n          {\n            \"Value\": \"2\"\n          },\n          {\n            \"Value\": \"3\"\n          }\n        ]\n      }\n    ]\n  },\n  {\n    \"Exit\": {\n      \"Var\": \"var\"\n    }\n  }\n]", 
        out,
    );
}

#[test]
fn test_print() {
    let output = Command::new("bash")
        .arg("test_scripts/test_compile.sh")
        .arg("test_scripts/print.zb")
        .output()
        .expect("failed");
    let out_slice = &String::from_utf8_lossy(&output.stdout)
        .replace("\0", "")
        .rsplit_once("...")
        .unwrap()
        .1
        .rsplit("\n")
        .map(|s| s.to_string())
        .collect::<Vec<_>>()[2..9];
    assert_eq!(out_slice, ["0", "11", "10", "321", "201", "1", "42"])
}
