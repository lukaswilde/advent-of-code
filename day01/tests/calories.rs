use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day01")?;
    cmd.arg(
        "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Most calories that an elve is carrying: 24000",
        ))
        .stdout(predicate::str::contains(
            "Calories by top three elves are: 45000",
        ));

    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day01")?;
    cmd.args(["-i", "../day01/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Most calories that an elve is carrying: 67633",
        ))
        .stdout(predicate::str::contains(
            "Calories by top three elves are: 199628",
        ));

    Ok(())
}
