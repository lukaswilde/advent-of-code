use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day15")?;
    cmd.arg(
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The number of blocked places is 26",
        ))
        .stdout(predicate::str::contains(
            "The tuning frequency of the distress beacon is 56000011",
        ));
    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day15")?;
    cmd.args(["-i", "../day15/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The number of blocked places is 4886370",
        ))
        .stdout(predicate::str::contains(
            "The tuning frequency of the distress beacon is 11374534948438",
        ));
    Ok(())
}
