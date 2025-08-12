pub mod compile_time_conf;
pub mod header;
pub mod navbar;
pub mod non_volatile_chunk;

use std::fs;
use std::process::exit;

use crate::header::*;
use crate::navbar::*;

use toml::*;

fn main() {
    let hello = NavTemplate {
        nav_items: vec![NavItem {
            name: "Index".to_string(),
            url: "https://example.com/index".to_string(),
        }],
    };
    let header = DefaultDarkHeader {
        global_title: "Example Forum",
        global_sub: "Discussion for TOPIC!",
        nav: &hello,
    };
    DefaultDarkHeader::save_to_disk(&header).unwrap();
    let config_contents = match fs::read_to_string("Config.toml") {
        Ok(c) => c,
        Err(_) => {
            eprintln!("couldn't read!");
            exit(1);
        }
    };
    let nav_array_p = toml::from_str::<Table>(config_contents.as_str()).unwrap();
    let nav_array = nav_array_p
        .get("global")
        .unwrap()
        .get("nav_links")
        .unwrap()
        .as_array()
        .unwrap();
    let mut nav_vec = vec![];
    for item in nav_array {
        nav_vec.push(NavItem {
            name: item.as_array().unwrap()[0].to_string(),
            url: item.as_array().unwrap()[1].to_string(),
        });
    }
    println!("{:?}", nav_vec);
    let navtest = NavTemplate { nav_items: nav_vec };

    println!("{}", navtest.render().unwrap());
}
