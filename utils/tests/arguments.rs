use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn text_as_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("utils")?;
    let argument = "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000"
        .trim()
        .replace(' ', "");

    cmd.arg(&argument);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(argument));

    Ok(())
}

#[test]
fn file_as_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("utils")?;
    let argument = "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000"
        .trim()
        .replace(' ', "");

    cmd.arg("-i").arg("tests/file_argument.txt");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(argument));
    Ok(())
}
