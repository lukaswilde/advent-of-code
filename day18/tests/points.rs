use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day18")?;
    cmd.arg(
        "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The number of free sides is 64"))
        .stdout(predicate::str::contains(
            "The number of exposed sides to water is 58",
        ));
    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day18")?;
    cmd.args(["-i", "../day18/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The number of free sides is 3564"))
        .stdout(predicate::str::contains(
            "The number of exposed sides to water is 2106",
        ));
    Ok(())
}
