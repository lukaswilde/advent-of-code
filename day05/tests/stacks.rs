use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn stacks() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day05")?;
    cmd.arg(
        "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The crates on top are CMZ"))
        .stdout(predicate::str::contains(
            "The alternative crates on top are MCD",
        ));

    Ok(())
}
