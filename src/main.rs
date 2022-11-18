// Copyright 2022 EagleOnGitHub
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use std::fs;
use std::io::Read; // i feel mental pain seeing "owo" in a crate

fn handle_command(s: String) {
    let mut acc: i128 = 0;
    let mut dat: Vec<i128> = vec!(16384);
    let mut dat_pointer = 0;
    let mut backwards_label = 0;
    let mut forwards_label = 0;
    let mut c;
    let mut u;
    let mut i = 0;
    while i < s.len() {
        u = s.as_bytes()[i];
        c = char::from_u32(u.into()).unwrap();
        match c {
            '+' => acc += 1,
            '-' => acc -= 1,
            '*' => acc *= dat[dat_pointer],
            '/' => acc /= dat[dat_pointer],
            '[' => dat[dat_pointer] += 1,
            ']' => dat[dat_pointer] -= 1,
            ';' => dat[dat_pointer] *= acc,
            '\'' => dat[dat_pointer] /= acc,
            'o' => print!("{}", char::from_u32(acc as u32).unwrap()),
            'p' => print!("{}", char::from_u32(dat[dat_pointer] as u32).unwrap()),
            'k' => print!("{}", acc),
            'l' => print!("{}", dat[dat_pointer]),
            ' ' => print!(" "),
            'e' => println!(),
            'f' => forwards_label = i,
            'b' => backwards_label = i,
            'g' => {
                if acc == dat[dat_pointer] {
                    i = forwards_label
                }
            }
            'n' => {
                if acc == dat[dat_pointer] {
                    i = backwards_label
                }
            }
            'h' => {
                if acc > dat[dat_pointer] {
                    i = forwards_label
                }
            }
            'm' => {
                if acc > dat[dat_pointer] {
                    i = backwards_label
                }
            }
            'j' => {
                if acc < dat[dat_pointer] {
                    i = forwards_label
                }
            }
            '<' => {
                if acc < dat[dat_pointer] {
                    i = backwards_label
                }
            },
            'q' => dat_pointer -= 1,
            'w' => dat_pointer += 1,
            _ => todo!(),
        }
        i += 1;
    }
}

fn repl() {
    let mut rl = Editor::<()>::new().expect("failed creating line reader");
    if rl.load_history("history.txt").is_err() {
        println!("no previous history");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                handle_command(line);
                println!();
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C pressed");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("EOF encountered or CTRL-D pressed");
                break;
            }
            Err(err) => {
                println!("unknown error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").expect("failed to write history file");
}

fn main() {
    color_backtrace::install();

    let mut contents = String::new();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut file = fs::File::open(&args[1]).expect("failed opening file");
        file.read_to_string(&mut contents)
            .expect("failed reading file to string");
        handle_command(contents)
    } else {
        repl()
    }
}
