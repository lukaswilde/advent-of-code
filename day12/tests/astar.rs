use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day12")?;
    cmd.arg(
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The cheapest path to the goal has cost 31",
        ))
        .stdout(predicate::str::contains(
            "The shortest path from any start point to the goal has cost 29",
        ));

    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day12")?;
    cmd.args(["-i", "../day12/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The cheapest path to the goal has cost 504",
        ))
        .stdout(predicate::str::contains(
            "The shortest path from any start point to the goal has cost 500",
        ));

    Ok(())
}
