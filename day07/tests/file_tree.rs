use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn sums() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day07")?;
    cmd.arg(
        "$ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("The sum of directories is 95437"))
        .stdout(predicate::str::contains("The smallest delete is 24933642"));

    Ok(())
}
