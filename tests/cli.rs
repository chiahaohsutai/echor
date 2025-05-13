use assert_cmd::{Command, assert::OutputAssertExt};
use predicates::prelude;

#[test]
fn crash_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello")
        .arg("world")
        .assert()
        .success()
        .stdout(predicates::str::contains("hello world"));
}
