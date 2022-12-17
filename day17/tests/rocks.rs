use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day17")?;
    cmd.arg(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The maximum height is 3068"));
    // .stdout(predicate::str::contains("The maximum height is 1514285714288"));
    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day17")?;
    cmd.args(["-i", "../day17/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The maximum height is 3166"));
    // .stdout(predicate::str::contains("The maximum height is 3166"));
    Ok(())
}
