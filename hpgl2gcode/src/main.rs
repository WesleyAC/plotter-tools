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

use hpgl::{parse_commands, Command, Point};
use std::fs::File;
use std::io::prelude::*;

fn plot_points(points: Vec<Point>, xscale: f64, yscale: f64) {
    for point in points {
        println!(
            "G1 X{} Y{}",
            (point.x as f64) * xscale,
            (point.y as f64) * yscale
        );
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let mut file = File::open(args[1].clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let cmds = parse_commands(contents).unwrap();

    // scale of 0.076 is good for Recurse Center plotter -> NYCR plotter conversion
    let xscale = 0.076;
    let yscale = 0.076;

    for cmd in cmds {
        match cmd {
            Command::PenUp(points) => {
                println!("G90");
                println!("M107");
                println!("G4 P100");
                plot_points(points, xscale, yscale);
            }
            Command::PenDown(points) => {
                println!("G90");
                println!("M106");
                println!("G4 P100");
                plot_points(points, xscale, yscale);
            }
            Command::PlotAbsolute(points) => {
                println!("G90");
                plot_points(points, xscale, yscale);
            }
            Command::PlotRelative(points) => {
                println!("G91");
                plot_points(points, xscale, yscale);
            }
            Command::SelectPen(_) => {}
            Command::Initalize => {}
        }
    }
    Ok(())
}
