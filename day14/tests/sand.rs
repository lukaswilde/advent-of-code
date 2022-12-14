use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day14")?;
    cmd.arg(
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The number of rested sand is 24"))
        .stdout(predicate::str::contains(
            "The number of rests needed for stopping is 93",
        ));
    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day14")?;
    cmd.args(["-i", "../day14/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The number of rested sand is 665"))
        .stdout(predicate::str::contains(
            "The number of rests needed for stopping is 25434",
        ));
    Ok(())
}
