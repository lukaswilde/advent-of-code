use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day20")?;
    cmd.arg(
        "1
2
-3
3
-2
0
4",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The grove coordinates are 3"))
        .stdout(predicate::str::contains(
            "The grove coordinates using the decryption key are 1623178306",
        ));
    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day20")?;
    cmd.args(["-i", "../day20/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The grove coordinates are 13183"))
        .stdout(predicate::str::contains(
            "The grove coordinates using the decryption key are 6676132372578",
        ));
    Ok(())
}
