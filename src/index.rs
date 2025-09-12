use askama::Template;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::Error;
use std::io::Write;

use crate::category_list::DefaultCategoryList;
use crate::header::DefaultDarkHeader;
use crate::*;

#[derive(Template)]
#[template(path = "default/index.html")]
#[derive(Debug)]
pub struct DefaultIndex<'a> {
    pub sitename: String,
    pub header: DefaultDarkHeader<'a>,
    pub cat_list: DefaultCategoryList,
}

impl<'a> DefaultIndex<'a> {
    pub fn save_to_disk(idx: &DefaultIndex) -> Result<(), Error> {
        let cache_path = "_cache";
        DirBuilder::new()
            .recursive(true)
            .create(cache_path)
            .unwrap();

        let mut file = File::create("_cache/index_default_dark.html")?;
        file.write_all(idx.render().unwrap().as_bytes())?;
        Ok(())
    }
}
