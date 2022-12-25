use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day25")?;
    cmd.arg(
        "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122",
    );
    cmd.assert().success().stdout(predicate::str::contains(
        "The snafu number to enter is 2=-1=0",
    ));
    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day25")?;
    cmd.args(["-i", "../day25/puzzle.txt"]);
    cmd.assert().success().stdout(predicate::str::contains(
        "The snafu number to enter is 20=02=120-=-2110-0=1",
    ));
    Ok(())
}
