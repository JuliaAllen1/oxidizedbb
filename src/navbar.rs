pub use askama::Template;

#[derive(Template, Debug)]
#[template(path = "default/nav.html")]

pub struct NavTemplate {
    pub nav_items: Vec<NavItem>,
}
#[derive(Debug)]
pub struct NavItem {
    pub name: String,
    pub url: String,
}
