use assert_cmd::Command;
use predicates::prelude::*;

#[cfg(not(target_os = "windows"))]
#[test]
fn successful_1_time() {
    let mut cmd = Command::cargo_bin("rty").unwrap();

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

#[cfg(target_os = "windows")]
#[test]
fn successful_1_time() {
    let mut cmd = Command::cargo_bin("rty").unwrap();

    cmd.arg("echo abc")
        .arg("-c")
        .arg("2")
        .assert()
        .success()
        .stdout(predicate::eq("abc\r\n"));
}

#[test]
fn failed_2_time() {
    let mut cmd = Command::cargo_bin("rty").unwrap();

    cmd.arg("dummy")
        .arg("-c")
        .arg("2")
        .assert()
        .failure()
        .stderr(predicate::eq(
            r"rty: command not found 'dummy'
rty: command not found 'dummy'
",
        ));
}

#[test]
fn failed_2_time_with_double_hyphen() {
    let mut cmd = Command::cargo_bin("rty").unwrap();

    cmd.arg("-c")
        .arg("2")
        .arg("--")
        .arg("dummy")
        .assert()
        .failure()
        .stderr(predicate::eq(
            r"rty: command not found 'dummy'
rty: command not found 'dummy'
",
        ));
}

#[test]
fn sleep_one_time() {
    let mut cmd = Command::cargo_bin("rty").unwrap();

    let now = std::time::Instant::now();

    cmd.arg("dummy")
        .arg("-c")
        .arg("2")
        .arg("-i")
        .arg("0.5")
        .assert()
        .failure();

    assert!(now.elapsed() >= std::time::Duration::from_secs_f64(0.5))
}
