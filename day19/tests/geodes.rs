use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day19")?;
    cmd.arg(
"Blueprint 1: Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore.  Each clay robot costs 3 ore.  Each obsidian robot costs 3 ore and 8 clay.  Each geode robot costs 3 ore and 12 obsidian."
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The quality level of the blueprints is 33",
        ))
        .stdout(predicate::str::contains(
            "The product of largest nummber of geodes opened is 3472",
        ));
    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day19")?;
    cmd.args(["-i", "../day19/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The quality level of the blueprints is 1681",
        ))
        .stdout(predicate::str::contains(
            "The product of largest nummber of geodes opened is 5394",
        ));
    Ok(())
}
