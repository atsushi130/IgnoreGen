
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

pub struct DefaultIgnore;

impl DefaultIgnore {
    pub fn get_default_ignore() -> String {
        "\n\
        # Created by IgnoreGen\n\
        \n\
        ### Default ###\n\
        .DS_Store\n\
        *.swp\n\
        *.log\n\
        *.bak\n\
        *.tmp\n\
        ".to_string()
    }
}