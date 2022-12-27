use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day08")?;
    cmd.arg(
        "30373
    25512
    65332
    33549
    35390",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The number of visible trees is 21",
        ))
        .stdout(predicate::str::contains("The maximum scenic score is 8"));

    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day08")?;
    cmd.args(["-i", "../day08/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The number of visible trees is 1711",
        ))
        .stdout(predicate::str::contains(
            "The maximum scenic score is 301392",
        ));

    Ok(())
}
