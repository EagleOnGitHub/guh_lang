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

use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = fs::File::open(&args[1]).expect("failed opening file");
    let mut contents = String::new();
    let mut acc = 0;
    let mut dat = 0;
    let mut backwards_label = 0;
    let mut forwards_label = 0;
    let mut c = 'a';
    let mut u = 0;
    let mut i = 0;

    file.read_to_string(&mut contents)
        .expect("failed reading file to string");

    while i < contents.len() {
        u = contents.as_bytes()[i];
        c = char::from_u32(u.into()).unwrap();
        match c {
            '+' => acc += 1,
            '-' => acc -= 1,
            '*' => acc *= dat,
            '/' => acc /= dat,
            '[' => dat += 1,
            ']' => dat -= 1,
            ';' => dat *= acc,
            '\'' => dat /= acc,
            'o' => print!("{}", char::from_u32(acc).unwrap()),
            'p' => print!("{}", char::from_u32(dat).unwrap()),
            'k' => print!("{}", acc),
            'l' => print!("{}", dat),
            ' ' => print!(" "),
            'e' => print!("\n"),
            'f' => forwards_label = i,
            'b' => backwards_label = i,
            'g' => if acc == dat { i = forwards_label },
            'n' => if acc == dat { i = backwards_label },
            'h' => if acc > dat { i = forwards_label },
            'm' => if acc > dat { i = backwards_label },
            'j' => if acc < dat { i = forwards_label },
            '<' => if acc < dat { i = backwards_label },
            _ => todo!(),
        }
        i += 1;
    }
}
