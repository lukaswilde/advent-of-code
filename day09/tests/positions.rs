use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn positions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day09")?;
    cmd.arg(
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
    );
    cmd.assert().success().stdout(predicate::str::contains(
        "The number of visited positions is 13",
    ));

    Ok(())
}
