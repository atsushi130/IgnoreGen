
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use super::super::curl::easy::Easy;
use std::io::{self, Write};
use super::Languages;

pub struct IgnoreGen;
impl IgnoreGen {
    pub fn generate(&self, languages: Languages) -> String {
        let mut easy  = Easy::new();
        let query     = &languages.join();
        let base_url  = "https://www.gitignore.io/api/";
        let url       = &self.build_url(base_url, query);

        let mut gitignore = Vec::new();

        easy.url(url).unwrap();

        {
            let mut transfer = easy.transfer();
            transfer.write_function(|data| {
                gitignore.extend_from_slice(data);
                Ok(io::stdout().write(data).unwrap())
            }).unwrap();
            transfer.perform().unwrap();
        }

        String::from_utf8(gitignore).unwrap()
    }

    fn build_url(&self, base_url: &str, query: &str) -> String {
        base_url.to_string() + query
    }
}
