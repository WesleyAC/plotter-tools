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

use hpgl::{parse_commands, Command, Point};
use std::fs::File;
use std::io::prelude::*;

fn draw_line(start: Point, end: Point, color: u8) {
    println!(
        "<line x1='{}' y1='{}' x2='{}' y2='{}' style='stroke:{};stroke-width:10'/>",
        // not sure why these have to be swapped - definitely a bug here :(
        start.y,
        start.x,
        end.y,
        end.x,
        &["", "black", "red", "blue", "green", "yellow", "orange", "brown", "pink"][color as usize],
    );
}

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let mut file = File::open(args[1].clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let commands = parse_commands(contents).unwrap();

    let mut color: u8 = 0;
    let mut position: Point = Point { x: 0, y: 0 };
    let mut pen_down: bool = false;

    println!("<html><body><svg viewBox='0 0 7650 10300'>");
    for cmd in commands {
        match cmd {
            Command::PenUp(points) => {
                pen_down = false;
                position = *points.last().unwrap_or(&position);
            }
            Command::PenDown(points) => {
                for p in points {
                    if pen_down && color != 0 {
                        draw_line(position, p, color.clone());
                    }
                    position = p;
                    pen_down = true;
                }
                pen_down = true;
            }
            Command::PlotAbsolute(points) => {
                for p in points {
                    if pen_down && color != 0 {
                        draw_line(position, p, color.clone());
                    }
                    position = p;
                }
            }
            Command::PlotRelative(points) => {
                for p in points {
                    let old_pos = position;
                    position.x += p.x;
                    position.y += p.y;
                    if pen_down && color != 0 {
                        draw_line(old_pos, position, color.clone());
                    }
                }
            }
            Command::SelectPen(c) => {
                color = c;
            }
            Command::Initalize => {}
        }
    }
    println!("</svg></body></html>");

    Ok(())
}
