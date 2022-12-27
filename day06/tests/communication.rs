use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn first() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day06")?;
    cmd.arg("bvwbjplbgvbhsrlpgdmjqwftvncz");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The first position after a packet marker is 5",
        ))
        .stdout(predicate::str::contains(
            "The first position after a message marker is 23",
        ));

    Ok(())
}

#[test]
fn second() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day06")?;
    cmd.arg("nppdvjthqldpwncqszvftbrmjlhg");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The first position after a packet marker is 6",
        ))
        .stdout(predicate::str::contains(
            "The first position after a message marker is 23",
        ));
    Ok(())
}

#[test]
fn third() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day06")?;
    cmd.arg("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The first position after a packet marker is 10",
        ))
        .stdout(predicate::str::contains(
            "The first position after a message marker is 29",
        ));
    Ok(())
}

#[test]
fn fourth() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day06")?;
    cmd.arg("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The first position after a packet marker is 11",
        ))
        .stdout(predicate::str::contains(
            "The first position after a message marker is 26",
        ));
    Ok(())
}

#[test]
fn fifth() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day06")?;
    cmd.arg("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The first position after a packet marker is 7",
        ))
        .stdout(predicate::str::contains(
            "The first position after a message marker is 19",
        ));
    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day06")?;
    cmd.args(["-i", "../day06/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The first position after a packet marker is 1896",
        ))
        .stdout(predicate::str::contains(
            "The first position after a message marker is 3452",
        ));
    Ok(())
}
