use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn sum_priorities() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day03")?;
    cmd.arg(
        "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The sum of priorities is 157"))
        .stdout(predicate::str::contains(
            "The sum of badge priorities is 70",
        ));

    Ok(())
}
