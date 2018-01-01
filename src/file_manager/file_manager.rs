
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
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
