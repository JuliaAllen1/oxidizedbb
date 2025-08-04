pub mod header;
pub mod navbar;
pub mod non_volatile_chunk;

use crate::header::*;
use crate::navbar::*;

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
    println!("{}", header.render().unwrap());
}
