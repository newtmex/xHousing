use std::io::Read;

use multiversx_sc_snippets::imports::*;

pub fn toml_into<T: for<'a> serde::Deserialize<'a>>(path: &str) -> T {
    let mut file = std::fs::File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    toml::from_str(&content).unwrap()
}
