pub mod compile_time_conf;
pub mod header;
pub mod navbar;
pub mod non_volatile_chunk;

use std::fs;
use std::process::exit;

use crate::compile_time_conf::*;

use toml::*;

fn main() {
    let config_contents = match fs::read_to_string("Config.toml") {
        Ok(c) => c,
        Err(_) => {
            eprintln!("couldn't read!");
            exit(1);
        }
    };
    generate_cached_blocks_by_toml(toml::from_str::<Table>(config_contents.as_str()).unwrap());
}
