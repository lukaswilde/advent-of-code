use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn positions_simple() -> Result<(), Box<dyn std::error::Error>> {
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
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The number of visited positions with 2 knots is 13",
        ))
        .stdout(predicate::str::contains(
            "The number of visited positions with 10 knots is 1",
        ));

    Ok(())
}

#[test]
fn positions_medium() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day09")?;
    cmd.arg(
        "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The number of visited positions with 2 knots is 88",
        ))
        .stdout(predicate::str::contains(
            "The number of visited positions with 10 knots is 36",
        ));

    Ok(())
}
