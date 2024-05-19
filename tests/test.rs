use assert_cmd::Command;
#[test]
fn run() {
    let mut cmd = Command::cargo_bin("name_value_pairs").unwrap();
    cmd.assert().success();
}