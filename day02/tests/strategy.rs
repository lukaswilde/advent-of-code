use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day02")?;
    cmd.arg(
        "A Y
    B X
    C Z",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The final score is 15"))
        .stdout(predicate::str::contains(
            "The final alternative score is 12",
        ));

    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day02")?;
    cmd.args(["-i", "../day02/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The final score is 14069"))
        .stdout(predicate::str::contains(
            "The final alternative score is 12411",
        ));

    Ok(())
}
