#![feature(or_patterns)]
use hpgl::{Point, Command, parse_commands};

// possible optimizations:
// * remove duplicate PU and PD commands
// * coalesce multiple PA, PU, and PD commands into one (is a superset of the above)
// * merge lines into one

fn main() {
    panic!("WIP code - not ready yet, sorry <3");
}

fn remove_duplicate_pen_commands(cmds: Vec<Command>) -> Vec<Command> {
    let mut out = vec![];
    let mut previous: Option<Command> = None;
    for cmd in cmds.iter() {
        match (previous, cmd) {
            (None, cmd) => previous = Some(cmd.clone()),
            (Some(Command::PenUp(_)) | Some(Command::PenDown(_)), Command::PenUp(pu)) => previous = Some(Command::PenUp(pu.to_vec())),
            (Some(Command::PenUp(_)) | Some(Command::PenDown(_)), Command::PenDown(pd)) => previous = Some(Command::PenDown(pd.to_vec())),
            (Some(prev), new) => {
                out.push(prev);
                previous = Some(new.clone());
            }
        }
    }
    if let Some(prev) = previous {
        out.push(prev);
    }
    out
}

#[test]
fn test_duplicates() {
    let test_commands = parse_commands(String::from("IN;PU;PD;PA1000,1000;PU;PU;")).unwrap();
    let expected_commands = parse_commands(String::from("IN;PD;PA1000,1000;PU;")).unwrap();
    let deduped = remove_duplicate_pen_commands(test_commands);
    assert_eq!(deduped, expected_commands);
}
