use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day22")?;
    cmd.arg(
        "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The final password is 6032"));
    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day22")?;
    cmd.args(["-i", "../day22/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The final password is 165094"))
        .stdout(predicate::str::contains(
            "The final password when seen as cube is 95316",
        ));
    Ok(())
}
