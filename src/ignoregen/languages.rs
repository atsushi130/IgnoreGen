
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

pub struct Languages {
    pub list: Vec<String>
}

impl Languages {

    pub fn new(list: &Vec<String>) -> Languages {
        Languages { list: list.to_vec() }
    }

    pub fn join(&self) -> String {
        self.list.join(",")
    }
}

