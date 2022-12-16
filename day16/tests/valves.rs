use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day16")?;
    cmd.arg(
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The maximal achievable flow is 1651",
        ))
        .stdout(predicate::str::contains(
            "The maximal achievable flow with elephant is 1707",
        ));
    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day16")?;
    cmd.args(["-i", "../day16/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "The maximal achievable flow is 2181",
        ))
        .stdout(predicate::str::contains(
            "The maximal achievable flow with elephant is 2824",
        ));
    Ok(())
}
