// Copyright 2019 Wesley Aptekar-Cassels
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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

fn plot_points(points: Vec<Point>) {
    for point in points {
        println!("G1 X{} Y{}", point.x, point.y);
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let mut file = File::open(args[1].clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for mut hpgl_cmd in contents.split(";") {
        hpgl_cmd = hpgl_cmd.trim();
        if hpgl_cmd.len() >= 2 {
            match parse_command(hpgl_cmd.to_string()) {
                Command::PenUp(points) => {
                    println!("G90");
                    println!("M107");
                    println!("G4 P100");
                    plot_points(points);
                }
                Command::PenDown(points) => {
                    println!("G90");
                    println!("M106");
                    println!("G4 P100");
                    plot_points(points);
                }
                Command::PlotAbsolute(points) => {
                    println!("G90");
                    plot_points(points);
                }
                Command::PlotRelative(points) => {
                    println!("G91");
                    plot_points(points);
                }
                Command::SelectPen(_) => {}
                Command::Initalize => {}
            }
        }
    }
    Ok(())
}
