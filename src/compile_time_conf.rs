use crate::header::DefaultDarkHeader;
use crate::navbar::NavItem;
use crate::navbar::NavTemplate;

use crate::header;

fn slice_first_and_last_char(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next(); // Consume the first character
    chars.next_back(); // Consume the last character
    chars.as_str() // Return the remaining slice
}

#[allow(dead_code)]

pub fn generate_cached_blocks_by_toml(config_file: toml::Table) -> Option<()> {
    let global_settings = config_file.get("global").unwrap();
    let nav_array = global_settings
        .get("nav_links")
        .unwrap()
        .as_array()
        .unwrap();
    let mut user_configured_navitems: Vec<NavItem> = vec![];
    for item in nav_array {
        user_configured_navitems.push(NavItem {
            name: slice_first_and_last_char(item.as_array().unwrap()[0].to_string().as_str())
                .to_string(),
            url: slice_first_and_last_char(item.as_array().unwrap()[1].to_string().as_str())
                .to_string(),
        });
    }
    let generated_navigation_template: NavTemplate = NavTemplate {
        nav_items: user_configured_navitems,
    };

    let generated_header = DefaultDarkHeader {
        global_title: global_settings.get("sitename").unwrap().as_str().unwrap(),
        global_sub: global_settings.get("tagline").unwrap().as_str().unwrap(),
        nav: &generated_navigation_template,
    };
    let _ = DefaultDarkHeader::save_to_disk(&generated_header);
    Some(())
}
