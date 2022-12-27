use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day03")?;
    cmd.arg(
        "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The sum of priorities is 157"))
        .stdout(predicate::str::contains(
            "The sum of badge priorities is 70",
        ));

    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day03")?;
    cmd.args(["-i", "../day03/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The sum of priorities is 7581"))
        .stdout(predicate::str::contains(
            "The sum of badge priorities is 2525",
        ));

    Ok(())
}
