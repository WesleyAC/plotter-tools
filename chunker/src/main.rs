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
use std::io::{Write, BufReader, BufRead, Error};
use std::time::Duration;

use serialport::prelude::*;

fn main() -> Result<(),Error> {
	let s = SerialPortSettings {
		baud_rate: 9600,
		data_bits: DataBits::Eight,
		flow_control: FlowControl::None,
		parity: Parity::None,
		stop_bits: StopBits::One,
		timeout: Duration::from_millis(1000),
	};

	let args: Vec<_> = std::env::args().collect();

	let input = File::open(&args[2])?;
	let buffered = BufReader::new(input);
	let mut cmds: Vec<Vec<u8>> = vec![];
	for cmd in buffered.lines() {
        cmds.push(cmd?.as_bytes().to_vec());
	}

    match serialport::open_with_settings(&args[1], &s) {
        Ok(mut port) => {
			port.write(b"IN;");
            let mut next_cmd = vec![];
			for cmd in cmds.iter() {
                if next_cmd.len() + cmd.len() < 57 {
                    next_cmd.append(&mut cmd.clone());
                } else {
                    port.write(&next_cmd);
                    println!("{}", String::from_utf8(next_cmd.to_vec()).unwrap());
                    port.write(b"OA;");
                    let mut c = 0;
                    while c != 13 {
                        let mut v = vec![0];
                        port.read(v.as_mut_slice());
                        c = v[0];
                    }
                    port.clear(ClearBuffer::All);
                    next_cmd = cmd.to_vec();
                }
			}
            port.write(&next_cmd);
            println!("{}", String::from_utf8(next_cmd.to_vec()).unwrap());
        }
        Err(e) => {
            ::std::process::exit(1);
        }
    };

	Ok(())
}
