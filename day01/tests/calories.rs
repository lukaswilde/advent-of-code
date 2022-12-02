use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn example() -> Result<(), Box<dyn std::error::Error>> {
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
        .stdout(predicate::str::contains("24000"))
        .stdout(predicate::str::contains("45000"));

    Ok(())
}
