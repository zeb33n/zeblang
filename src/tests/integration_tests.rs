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

#[test]
fn test_while() {
    let output = Command::new("bash")
        .arg("test_compile.sh")
        .arg("test_scripts/while.zb")
        .output()
        .expect("failed");
    let out = extract_value_from_stdout(&output.stdout);
    assert_eq!(12, out);
}

#[test]
fn test_precedance() {
    let output = Command::new("bash")
        .arg("test_compile.sh")
        .arg("test_scripts/precedance.zb")
        .output()
        .expect("failed");
    let out = extract_value_from_stdout(&output.stdout);
    assert_eq!(13, out);
}

#[test]
fn test_variables() {
    let output = Command::new("bash")
        .arg("test_compile.sh")
        .arg("test_scripts/variables.zb")
        .output()
        .expect("failed");
    let out = extract_value_from_stdout(&output.stdout);
    assert_eq!(3, out);
}
