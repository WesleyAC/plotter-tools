use std::fs::File;
use std::io::prelude::*;

struct Point {
    x: i32,
    y: i32,
}

enum Command {
    PenUp,
    PenDown,
    PlotAbsolute(Point),
    PlotRelative(Point),
    SelectPen(u8),
}

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let mut file = File::open(args[1].clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut commands: Vec<Command> = vec![];

    for mut cmd in contents.split(";") {
        cmd = cmd.trim();
        if cmd.len() >= 2 {
            let cmd_type: String = cmd.trim()[0..2].to_string();
            if cmd_type == "PU" {
                commands.push(Command::PenUp);
            } else if cmd_type == "PD" {
                commands.push(Command::PenDown);
            } else if cmd_type == "PA" {
                let coords_part = cmd[2..].to_string();
                let coords: Vec<_> = coords_part.split(",").collect();
                commands.push(Command::PlotAbsolute(
                        Point {
                            x: coords[0].parse().unwrap(),
                            y: coords[1].parse().unwrap(),
                        }));
            } else if cmd_type == "PR" {
                let coords_part = cmd[2..].to_string();
                let coords: Vec<_> = coords_part.split(",").collect();
                commands.push(Command::PlotRelative(
                        Point {
                            x: coords[0].parse().unwrap(),
                            y: coords[1].parse().unwrap(),
                        }));
            } else if cmd_type == "SP" {
                let pen = cmd[2..].to_string().parse().unwrap();
                commands.push(Command::SelectPen(pen));
            }
        }
    }

    let mut color: u8 = 0;
    let mut position: Point = Point { x: 0, y: 0 };
    let mut pen_down: bool = false;

    println!("<html><body><svg viewBox='0 0 7650 10300'>");
    for cmd in commands {
        match cmd {
            Command::PenUp => {
                pen_down = false;
            },
            Command::PenDown => {
                pen_down = true;    
            },
            Command::PlotAbsolute(p) => {
                if pen_down {
                    println!(
                        "<line x1='{}' y1='{}' x2='{}' y2='{}' style='stroke:rgb(0,0,0);stroke-width:2'/>",
                        position.y,
                        position.x,
                        p.y,
                        p.x
                    );
                }
                position = p;
            },
            Command::PlotRelative(p) => {
                if pen_down {
                    println!(
                        "<line x1='{}' y1='{}' x2='{}' y2='{}' style='stroke:rgb(0,0,0);stroke-width:2'/>",
                        position.y,
                        position.x,
                        position.y + p.y,
                        position.x + p.x
                    );
                }
                position.x += p.x;
                position.y += p.y;
            },
            Command::SelectPen(c) => {
                color = c;
            }
        }
    }
    println!("</svg></body></html>");

    Ok(())
}
