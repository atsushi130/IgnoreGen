
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate curl;

use std::env;

mod ignoregen;
use ignoregen::IgnoreGen;
use ignoregen::Languages;

mod file_manager;
use file_manager::FileManager;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1..].to_vec().is_empty() {
        println!("not found arguments.");
        return;
    }

    let languages = Languages::new(&args[1..].to_vec());
    let gitignore = IgnoreGen.generate(languages);
    FileManager.write(gitignore.as_bytes());
}
