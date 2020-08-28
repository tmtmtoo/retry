use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn successful_1_time() {
    let mut cmd = Command::cargo_bin("retry").unwrap();

    cmd.arg("echo abc")
        .arg("-c")
        .arg("2")
        .assert()
        .success()
        .stdout(predicate::eq(
            r"abc
",
        ));
}

#[test]
fn failed_2_time() {
    let mut cmd = Command::cargo_bin("retry").unwrap();

    cmd.arg("dummy")
        .arg("-c")
        .arg("2")
        .assert()
        .success()
        .stderr(predicate::eq(
            r"retry: command not found 'dummy'
retry: command not found 'dummy'
",
        ));
}

#[test]
fn sleep_one_time() {
    let mut cmd = Command::cargo_bin("retry").unwrap();

    let now = std::time::Instant::now();

    cmd.arg("dummy")
        .arg("-c")
        .arg("2")
        .arg("-i")
        .arg("0.5")
        .assert()
        .success();

    assert!(now.elapsed() >= std::time::Duration::from_secs_f64(0.5))
}
