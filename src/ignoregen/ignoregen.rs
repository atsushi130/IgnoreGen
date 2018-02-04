
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use super::super::https_client::HttpsClientBuilder;
use std::io::Read;
use super::Languages;
use super::DefaultIgnore;

pub struct IgnoreGen;
impl IgnoreGen {
    pub fn generate(&self, languages: Languages) -> String {

        let mut response = String::new();

        let base_url  = "https://www.gitignore.io/api/";
        let url = self.build_url(base_url, &languages.join());

        HttpsClientBuilder::build()
            .get(&url)
            .send()
            .unwrap()
            .read_to_string(&mut response)
            .unwrap();

        DefaultIgnore::get_default_ignore() + response.as_str()
    }

    fn build_url(&self, base_url: &str, query: &str) -> String {
        base_url.to_string() + query
    }
}
