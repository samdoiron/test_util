use rustc_serialize::json;

pub fn strange_unicode_strings() -> Vec<String> {
    let naughty_strings = include_str!("blns.json");

    // This parse should *never* fail, so it's Ok to just unwrap.
    let parsed = json::decode(naughty_strings).unwrap();
    return parsed;
}
