use std::fs::{DirBuilder, File};
use std::io::{Error, Write};

use crate::navbar::*;

#[derive(Template)]
#[template(path = "default/header.html")]
pub struct DefaultDarkHeader<'a> {
    pub global_title: &'a str,
    pub global_sub: &'a str,
    pub nav: &'a NavTemplate,
}

impl<'a> DefaultDarkHeader<'a> {
    pub fn save_to_disk(hdr: &DefaultDarkHeader) -> Result<(), Error> {
        let cache_path = "_cache";
        DirBuilder::new()
            .recursive(true)
            .create(cache_path)
            .unwrap();

        let mut file = File::create("_cache/header_default_dark.html")?;
        file.write_all(hdr.render().unwrap().as_bytes())?;
        Ok(())
    }
}
