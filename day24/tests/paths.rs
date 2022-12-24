use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day24")?;
    cmd.arg(
        "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The shortest path takes 18 minutes",
        ))
        .stdout(predicate::str::contains(
            "Going back and reaching the goal again takes 54 minutes",
        ));
    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day24")?;
    cmd.args(["-i", "../day24/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The shortest path takes 240 minutes",
        ))
        .stdout(predicate::str::contains(
            "Going back and reaching the goal again takes 717 minutes",
        ));
    Ok(())
}
