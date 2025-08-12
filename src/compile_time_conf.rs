use crate::header;
use crate::navbar;
use crate::navbar::NavTemplate;
use toml::*;
#[allow(dead_code)]

fn generate_cached_blocks_by_toml(config_file: toml::Table) -> Option<()> {
    let global_config = config_file.get("global").unwrap();
    let generated_navigation_template: NavTemplate = NavTemplate { nav_items: vec![] };
    Some(())
}
