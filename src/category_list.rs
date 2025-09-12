use askama::Template;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::Error;
use std::io::Write;

#[derive(Template)]
#[template(path = "default/category_list.html")]
#[derive(Debug)]
pub struct DefaultCategoryList {
    pub category_listing: Vec<Category>,
}
#[derive(Debug)]
pub struct Category {
    pub name: String,
}

impl DefaultCategoryList {
    pub fn save_to_disk(catl: &DefaultCategoryList) -> Result<(), Error> {
        let cache_path = "_cache";
        DirBuilder::new()
            .recursive(true)
            .create(cache_path)
            .unwrap();

        let mut file = File::create("_cache/category_list_default_dark.html")?;
        file.write_all(catl.render().unwrap().as_bytes())?;
        Ok(())
    }
}
