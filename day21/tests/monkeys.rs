use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day21")?;
    cmd.arg(
        "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The root monkey will yell 152"))
        .stdout(predicate::str::contains("We need to yell 301"));
    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day21")?;
    cmd.args(["-i", "../day21/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The root monkey will yell 194501589693264",
        ))
        .stdout(predicate::str::contains("We need to yell 3887609741189"));
    Ok(())
}
