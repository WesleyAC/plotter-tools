// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Command {
    PenUp(Vec<Point>),
    PenDown(Vec<Point>),
    PlotAbsolute(Vec<Point>),
    PlotRelative(Vec<Point>),
    SelectPen(u8),
    Initalize,
}

// TODO: If you edit this, split it into a separate mod and share between viz and hpgl2gcode
fn parse_command(cmd: String) -> Command {
    let cmd_type: String = cmd[0..2].to_string();
    let mut points: Vec<Point> = vec![];
    if cmd.len() > 2 && cmd_type != "SP" {
        let coords_part: String = cmd[2..].to_string();
        let coords: Vec<_> = coords_part.split(",").collect();
        if coords.len() % 2 != 0 {
            panic!("Odd number of points given to command!");
        }
        for i in 0..coords.len()/2 {
            points.push(Point {
                x: coords[i*2].trim().parse().unwrap(),
                y: coords[i*2+1].trim().parse().unwrap(),
            });
        }
    }
    if cmd_type == "PU" {
        Command::PenUp(points)
    } else if cmd_type == "PD" {
        Command::PenDown(points)
    } else if cmd_type == "PA" {
        Command::PlotAbsolute(points)
    } else if cmd_type == "PR" {
        Command::PlotRelative(points)
    } else if cmd_type == "SP" {
        let pen = cmd[2..].to_string().trim().parse().unwrap();
        Command::SelectPen(pen)
    } else if cmd_type == "IN" {
        Command::Initalize
    } else {
        panic!("unknown command")
    }
}

fn draw_line(start: Point, end: Point, color: u8) {
    println!(
        "<line x1='{}' y1='{}' x2='{}' y2='{}' style='stroke:{};stroke-width:2'/>",
        // not sure why these have to be swapped - definitely a bug here :(
        start.y,
        start.x,
        end.y,
        end.x,
        &["black", "red", "blue", "green", "yellow", "orange", "brown", "pink"][color as usize],
    );
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
            commands.push(parse_command(cmd.to_string()));
        }
    }

    let mut color: u8 = 0;
    let mut position: Point = Point { x: 0, y: 0 };
    let mut pen_down: bool = false;

    println!("<html><body><svg viewBox='0 0 7650 10300'>");
    for cmd in commands {
        match cmd {
            Command::PenUp(points) => {
                pen_down = false;
                position = *points.last().unwrap_or(&position);
            },
            Command::PenDown(points) => {
                for p in points {
                    if pen_down && color != 0 {
                        draw_line(position, p, color.clone());
                    }
                    position = p;
                    pen_down = true;
                }
                pen_down = true;
            },
            Command::PlotAbsolute(points) => {
                for p in points {
                    if pen_down && color != 0 {
                        draw_line(position, p, color.clone());
                    }
                    position = p;
                }
            },
            Command::PlotRelative(points) => {
                for p in points {
                    if pen_down && color != 0 {
                        draw_line(position, p, color.clone());
                    }
                    position.x += p.x;
                    position.y += p.y;
                }
            },
            Command::SelectPen(c) => {
                color = c;
            }
            Command::Initalize => {}
        }
    }
    println!("</svg></body></html>");

    Ok(())
}
