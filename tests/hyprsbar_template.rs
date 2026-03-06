use hyprsink::template::Template;
use std::fs;
use std::path::PathBuf;

#[test]
fn hyprsbar_template_has_required_metadata_header() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/templates/hyprsbar.tpl");
    let content = fs::read_to_string(path).expect("template file should exist");

    let tpl: Template = toml::from_str(&content).expect("template should parse");
    assert_eq!(tpl.manifest.name, "hyprsbar");
    assert_eq!(tpl.targets.len(), 1);

    let target = &tpl.targets[0];
    assert_eq!(target.target, "~/.config/hypr/hyprs/bar.conf");
    assert!(target.content.contains("# hypr metadata"));
    assert!(target.content.contains("# type = bar"));
}
