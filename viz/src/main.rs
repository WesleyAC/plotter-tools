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

use std::collections::HashMap;
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

    let mut colors: HashMap<u8, String> = HashMap::new();
    colors.insert(1, "black".to_string());
    colors.insert(2, "red".to_string());
    colors.insert(3, "blue".to_string());
    colors.insert(4, "green".to_string());
    colors.insert(5, "yellow".to_string());
    colors.insert(6, "orange".to_string());
    colors.insert(7, "brown".to_string());
    colors.insert(8, "pink".to_string());

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
                if pen_down && color != 0 {
                    println!(
                        "<line x1='{}' y1='{}' x2='{}' y2='{}' style='stroke:{};stroke-width:2'/>",
                        position.y,
                        position.x,
                        p.y,
                        p.x,
                        colors[&color],
                    );
                }
                position = p;
            },
            Command::PlotRelative(p) => {
                if pen_down && color != 0 {
                    println!(
                        "<line x1='{}' y1='{}' x2='{}' y2='{}' style='stroke:{};stroke-width:2'/>",
                        position.y,
                        position.x,
                        position.y + p.y,
                        position.x + p.x,
                        colors[&color],
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
