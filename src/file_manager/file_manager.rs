
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use std::io::prelude::*;
use std::fs::File;

pub struct FileManager;

impl FileManager {
    pub fn write(&self, write_string: &[u8]) {
        let mut f = File::create(".gitignore").unwrap();
        f.write_all(write_string).unwrap();
    }
}
