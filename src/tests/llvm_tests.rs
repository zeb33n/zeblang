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
        .arg("test_scripts/test_compile_llvm.sh")
        .arg(addr)
        .output()
        .expect("failed");
    extract_value_from_stdout(&output.stdout)
}

#[test]
fn test_funcs() {
    let out = run_zeblang_file("test_scripts/funcs.zb");
    assert_eq!(18, out);
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
fn test_print() {
    let output = Command::new("bash")
        .arg("test_scripts/test_compile_llvm.sh")
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
