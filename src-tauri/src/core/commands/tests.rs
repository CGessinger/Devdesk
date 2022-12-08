use std::path::Path;

use super::custom::read_custom_scripts;

#[test]
pub fn read_scripts() {
    let path =
        Path::new("/Users/cgessinger/Documents/Programming/maintained/devdesk/dev-test/.devdesk");
    let scripts = read_custom_scripts(path);
    assert_eq!(scripts.len(), 1)
}
