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
use std::path::PathBuf;

use structopt::StructOpt;

fn plot_points(points: Vec<Point>, xscale: f64, yscale: f64) {
    for point in points {
        println!(
            "G1 X{} Y{}",
            (point.x as f64) * xscale,
            (point.y as f64) * yscale
        );
    }
}

#[derive(Debug, StructOpt)]
struct Args {
    file: PathBuf,
    #[structopt(long = "xscale", short = "x", default_value = "0.076")]
    xscale: f64,
    #[structopt(long = "yscale", short = "y", default_value = "0.076")]
    yscale: f64,
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();
    let mut file = File::open(args.file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let cmds = parse_commands(contents).unwrap();

    for cmd in cmds {
        match cmd {
            Command::PenUp(points) => {
                println!("G90");
                println!("M107");
                println!("G4 P100");
                plot_points(points, args.xscale, args.yscale);
            }
            Command::PenDown(points) => {
                println!("G90");
                println!("M106");
                println!("G4 P100");
                plot_points(points, args.xscale, args.yscale);
            }
            Command::PlotAbsolute(points) => {
                println!("G90");
                plot_points(points, args.xscale, args.yscale);
            }
            Command::PlotRelative(points) => {
                println!("G91");
                plot_points(points, args.xscale, args.yscale);
            }
            Command::SelectPen(_) => {}
            Command::Initialize => {}
        }
    }
    Ok(())
}
