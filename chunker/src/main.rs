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

extern crate serialport;

use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::path::PathBuf;
use std::time::{Instant, Duration};

use serialport::prelude::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Args {
    hpgl_file: PathBuf,
    #[structopt(
        help = "serial device to use, such as /dev/ttyUSB0. attempts to autodetect by default."
    )]
    serial_device: Option<PathBuf>,
    #[structopt(short = "b", default_value = "60")]
    buffer_size: usize,
    #[structopt(long = "baud", default_value = "9600")]
    baud_rate: u32,
    #[structopt(
        long = "timeout",
        default_value = "30000",
        help = "serial port timeout, in milliseconds"
    )]
    timeout: u64,
}

fn print_progress(percent: f64) {
    let num_dots = (percent * 80.0) as usize;
    println!(
        "\x1B[F{:3}% [{}{}]",
        (percent * 100.0) as usize,
        "♥".repeat(num_dots),
        " ".repeat(80 - num_dots)
    );
}

fn main() -> Result<(), Error> {
    let args = Args::from_args();

    let serial_device = args.serial_device.unwrap_or({
        let devs: Vec<String> = std::fs::read_dir("/dev/")
            .unwrap()
            .filter_map(|e| {
                let p = e.unwrap().path().to_str()?.to_string();
                if p.starts_with("/dev/ttyUSB") || p.starts_with("/dev/tty.usbserial") {
                    Some(p)
                } else {
                    None
                }
            })
            .collect();
        if devs.len() == 1 {
            let dev = PathBuf::from(devs[0].clone());
            println!("autodetected serial device: {:#?}", dev);
            dev
        } else if devs.len() == 0 {
            println!("couldn't detect serial device! do you have the driver installed?");
            ::std::process::exit(2);
        } else {
            println!(
                "detected multiple serial devices: {:#?}, please specify only one!",
                devs
            );
            ::std::process::exit(2);
        }
    });

    let s = SerialPortSettings {
        baud_rate: args.baud_rate,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(args.timeout),
    };

    let input = File::open(args.hpgl_file)?;
    let buffered = BufReader::new(input);
    let mut cmds: Vec<Vec<u8>> = vec![];
    for cmd in buffered.lines() {
        cmds.push(cmd?.as_bytes().to_vec());
    }

    println!("");
    let start_time = Instant::now();
    match serialport::open_with_settings(&serial_device, &s) {
        Ok(mut port) => {
            let mut next_cmd = vec![];
            for (i, cmd) in cmds.iter().enumerate() {
                if next_cmd.len() + cmd.len() < args.buffer_size - 3 {
                    next_cmd.append(&mut cmd.clone());
                } else {
                    port.write(&next_cmd)?;
                    port.write(b"OA;")?;
                    print_progress(i as f64 / cmds.len() as f64);
                    let mut c = 0;
                    while c != 13 {
                        let mut v = vec![0];
                        port.read(v.as_mut_slice())?;
                        c = v[0];
                    }
                    port.clear(ClearBuffer::All)?;
                    next_cmd = cmd.to_vec();
                }
            }
            port.write(&next_cmd)?;
            print_progress(1.0);
        }
        Err(e) => {
            println!("Error opening serial port {:#?}: {}", serial_device, e);
            ::std::process::exit(1);
        }
    };

    // Inaccurate - should send OA for last command and block until done.
    println!("{} seconds elapsed.", start_time.elapsed().as_secs());

    Ok(())
}
