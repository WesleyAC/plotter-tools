#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Command {
    PenUp(Vec<Point>),
    PenDown(Vec<Point>),
    PlotAbsolute(Vec<Point>),
    PlotRelative(Vec<Point>),
    SelectPen(u8),
    Initialize,
}

pub fn parse_commands(cmd_str: String) -> Result<Vec<Command>, Vec<String>> {
    let cmds = cmd_str
        .split(";")
        .map(|cmd| cmd.trim())
        .filter(|cmd| cmd.len() != 0);
    let maybe_parsed_cmds = cmds.map(|cmd| parse_command(cmd.to_string()));
    let any_failures = maybe_parsed_cmds
        .clone()
        .fold(false, |flag, curr| flag || curr.is_err());
    if any_failures {
        Err(maybe_parsed_cmds
            .filter_map(|cmd| match cmd {
                Ok(_) => None,
                Err(e) => Some(e),
            })
            .collect())
    } else {
        Ok(maybe_parsed_cmds
            .filter_map(|cmd| match cmd {
                Ok(c) => Some(c),
                Err(_) => None,
            })
            .collect())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CanonicalCommand {
    PenUp,
    PenDown,
    PlotAbsolute(Point),
    SelectPen(u8),
    Initialize,
}

/// Canonicalize a list of commands:
/// * PU and PD do not have coords associated
/// * All movements are absolute
pub fn canonicalize(cmds: Vec<Command>) -> Vec<CanonicalCommand> {
    let mut out = vec![];
    let mut absolute_coords = true;
    for cmd in cmds {
        match cmd {
            Command::PenUp(points) => {
                out.push(CanonicalCommand::PenUp);
                if absolute_coords {
                    if let Some(point) = points.last() {
                        out.push(CanonicalCommand::PlotAbsolute(*point));
                    }
                } else {
                    panic!("can't handle relative coords :(");
                }
            },
            Command::PenDown(points) => {
                out.push(CanonicalCommand::PenDown);
                if absolute_coords {
                    for point in points {
                        out.push(CanonicalCommand::PlotAbsolute(point));
                    }
                } else {
                    panic!("can't handle relative coords :(");
                }
            },
            Command::PlotAbsolute(points) => {
                for point in points {
                    out.push(CanonicalCommand::PlotAbsolute(point));
                }
            },
            Command::PlotRelative(_) => {
                panic!("can't handle relative coords :(");
            },
            Command::SelectPen(pen) => {
                out.push(CanonicalCommand::SelectPen(pen));
            },
            Command::Initialize => {
                out.push(CanonicalCommand::Initialize);
            }
        }
    }
    out
}

pub fn canonical_commands_to_string(cmds: Vec<CanonicalCommand>) -> String {
    let mut out = String::new();
    for cmd in cmds{
        match cmd{
            CanonicalCommand::PenUp => out.push_str("PU;\n"),
            CanonicalCommand::PenDown => out.push_str("PD;\n"),
            CanonicalCommand::PlotAbsolute(point) => out.push_str(&format!("PA{},{};\n", point.x, point.y)),
            CanonicalCommand::SelectPen(pen) => out.push_str(&format!("SP{};\n", pen)),
            CanonicalCommand::Initialize => out.push_str("IN;\n"),
        }
    }
    out
}

fn parse_command(cmd: String) -> Result<Command, String> {
    if cmd.len() < 2 {
        return Err(cmd);
    }
    let cmd_type: String = cmd[0..2].to_string();
    let mut points: Vec<Point> = vec![];
    if cmd.len() > 2 && cmd_type != "SP" {
        let coords_part: String = cmd[2..].to_string();
        let coords: Vec<_> = coords_part.split(",").collect();
        if coords.len() % 2 != 0 {
            return Err(cmd);
        }
        for i in 0..coords.len() / 2 {
            points.push(Point {
                x: coords[i * 2].trim().parse().map_err(|_| cmd.clone())?,
                y: coords[i * 2 + 1].trim().parse().map_err(|_| cmd.clone())?,
            });
        }
    }
    if cmd_type == "PU" {
        Ok(Command::PenUp(points))
    } else if cmd_type == "PD" {
        Ok(Command::PenDown(points))
    } else if cmd_type == "PA" {
        Ok(Command::PlotAbsolute(points))
    } else if cmd_type == "PR" {
        Ok(Command::PlotRelative(points))
    } else if cmd_type == "SP" {
        let pen = cmd[2..].to_string().trim().parse().unwrap();
        Ok(Command::SelectPen(pen))
    } else if cmd_type == "IN" {
        Ok(Command::Initialize)
    } else {
        Err(cmd)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_parse_command(cmd: &str, expected: Option<Command>) {
        let parsed = parse_command(cmd.to_string());
        match expected {
            Some(cmd) => assert_eq!(parsed, Ok(cmd)),
            None => assert_eq!(parsed, Err(cmd.to_string())),
        }
    }

    #[test]
    fn test_parse_multiple_commands() {
        assert_eq!(
            parse_commands("  PU  ; PD  ; ".to_string()),
            Ok(vec![Command::PenUp(vec![]), Command::PenDown(vec![])])
        );
        assert_eq!(parse_commands("".to_string()), Ok(vec![]));
        assert_eq!(parse_commands("\n".to_string()), Ok(vec![]));
        assert_eq!(parse_commands("  ".to_string()), Ok(vec![]));
        assert_eq!(
            parse_commands("  PU  ; command_not_valid ; PD  ; ".to_string()),
            Err(vec!["command_not_valid".to_string()])
        );
    }

    #[test]
    fn test_parser() {
        check_parse_command("", None);
        check_parse_command("a", None);
        check_parse_command("command_not_valid", None);
        check_parse_command("PU ", None);
        check_parse_command(" PU", None);
        check_parse_command("PA foo, bar", None);
        check_parse_command("PU", Some(Command::PenUp(vec![])));
        // TODO: check commands that take points
    }

    #[test]
    fn test_canonicalize() {
        let cmds = canonicalize(parse_commands(include_str!("../testdata/canonicalize.hpgl").to_string()).unwrap());
        assert_eq!(
            cmds,
            vec![
                CanonicalCommand::SelectPen(1),
                CanonicalCommand::PlotAbsolute(Point { x: 1000, y: 1000 }),
                CanonicalCommand::PenDown,
                CanonicalCommand::PlotAbsolute(Point { x: 1000, y: 2000 }),
                CanonicalCommand::PlotAbsolute(Point { x: 2000, y: 2000 }),
                CanonicalCommand::PlotAbsolute(Point { x: 2000, y: 1000 }),
                CanonicalCommand::PlotAbsolute(Point { x: 1000, y: 1000 }),
                CanonicalCommand::PenUp,
                CanonicalCommand::PlotAbsolute(Point { x: 0, y: 0 }),
                CanonicalCommand::PenDown,
                CanonicalCommand::PlotAbsolute(Point { x: 5000, y: 5000 }),
                CanonicalCommand::PenUp,
            ]
        );
    }
}
