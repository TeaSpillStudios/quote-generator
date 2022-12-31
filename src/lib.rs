use ron::to_string;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};
use xdg::BaseDirectories;

#[derive(Serialize, Deserialize)]
pub struct QuoteGenerator {
    quotes: HashMap<String, String>,
}

impl QuoteGenerator {
    pub fn save_to_file(&self) {
        let data = to_string(&self).unwrap();

        let base_dirs = BaseDirectories::with_prefix("quote_generator").unwrap();

        let conf = base_dirs
            .place_data_file("quotes.ron")
            .expect("Cannot create the configuration directory.");

        let mut conf = File::create(conf).unwrap();

        conf.write_all(data.as_bytes())
            .expect("Failed to write data.")
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
