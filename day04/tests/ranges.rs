use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn sum_priorities() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day04")?;
    cmd.arg(
        "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The number of completely overlapping ranges is 2",
        ))
        .stdout(predicate::str::contains(
            "The number of overlapping ranges is 4",
        ));

    Ok(())
}
