pub use askama::Template;

#[derive(Template)]
#[template(path = "default/nav.html")]

pub struct NavTemplate {
    pub nav_items: Vec<NavItem>,
}

pub struct NavItem {
    pub name: String,
    pub url: String,
}
