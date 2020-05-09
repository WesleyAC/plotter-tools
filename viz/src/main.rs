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
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

fn draw_line(start: Point, end: Point, color: u8, orientation: Orientation, max_y: i32) {
    let (x1, y1, x2, y2) = match orientation {
        Orientation::Portrait => (start.y, start.x, end.y, end.x),
        Orientation::Landscape => (start.x, max_y-start.y, end.x, max_y-end.y),
    };
    println!(
        "<line x1='{}' y1='{}' x2='{}' y2='{}' style='stroke:{};stroke-width:10'/>",
        x1, y1, x2, y2,
        &["", "black", "red", "blue", "green", "yellow", "orange", "brown", "pink"][color as usize],
    );
}

#[derive(Debug, StructOpt)]
struct Args {
    file: PathBuf,
    #[structopt(long, default_value="7440", help="Plotter model. Options: '7440', '7475a3', '7475a4'")]
    model: Model,
    #[structopt(long, default_value="portrait", help="Display orientation. Options: 'portrait', 'landscape'")]
    orientation: Orientation,
}

#[derive(Copy, Clone, Debug)]
enum Model {
    Hp7440,
    Hp7475A3,
    Hp7475A4,
}

impl FromStr for Model {
    type Err = String;
    fn from_str(model: &str) -> Result<Self, Self::Err> {
        match model {
            "7440" => Ok(Model::Hp7440),
            "7475a3" => Ok(Model::Hp7475A3),
            "7475a4" => Ok(Model::Hp7475A4),
            _ => Err("Could not parse model".to_string()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Orientation {
    Portrait,
    Landscape,
}

impl FromStr for Orientation {
    type Err = String;
    fn from_str(model: &str) -> Result<Self, Self::Err> {
        match model {
            "portrait" => Ok(Orientation::Portrait),
            "landscape" => Ok(Orientation::Landscape),
            _ => Err("Could not parse orientation".to_string()),
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();

    let hpgl_file = fs::read_to_string(args.file)?;

    let commands = parse_commands(hpgl_file).unwrap();

    let mut color: u8 = 0;
    let mut position: Point = Point { x: 0, y: 0 };
    let mut pen_down: bool = false;

    let (x, y) = match args.model {
        Model::Hp7440 => (7650, 10300),
        Model::Hp7475A3 => (10365, 16640),
        Model::Hp7475A4 => (7962, 10365),
    };
    match args.orientation {
        Orientation::Portrait => println!("<html><body><svg viewBox='0 0 {} {}'>", x, y),
        Orientation::Landscape => println!("<html><body><svg viewBox='0 0 {} {}'>", y, x),
    };

    for cmd in commands {
        match cmd {
            Command::PenUp(points) => {
                pen_down = false;
                position = *points.last().unwrap_or(&position);
            }
            Command::PenDown(points) => {
                for p in points {
                    if pen_down && color != 0 {
                        draw_line(position, p, color.clone(), args.orientation, x);
                    }
                    position = p;
                    pen_down = true;
                }
                pen_down = true;
            }
            Command::PlotAbsolute(points) => {
                for p in points {
                    if pen_down && color != 0 {
                        draw_line(position, p, color.clone(), args.orientation, x);
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
                        draw_line(old_pos, position, color.clone(), args.orientation, x);
                    }
                }
            }
            Command::SelectPen(c) => {
                color = c;
            }
            Command::Initialize => {}
        }
    }
    println!("</svg></body></html>");

    Ok(())
}
