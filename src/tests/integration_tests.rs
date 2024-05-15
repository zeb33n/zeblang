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
        .arg("test_compile.sh")
        .arg(addr)
        .output()
        .expect("failed");
    extract_value_from_stdout(&output.stdout)
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
