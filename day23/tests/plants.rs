use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day23")?;
    cmd.arg(
        "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The number of empty tiles is 110"))
        .stdout(predicate::str::contains(
            "The first round no elve moves is 20",
        ));
    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day23")?;
    cmd.args(["-i", "../day23/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The number of empty tiles is 4181",
        ))
        .stdout(predicate::str::contains(
            "The first round no elve moves is 973",
        ));
    Ok(())
}
