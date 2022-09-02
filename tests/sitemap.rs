use chrono::{DateTime, Utc};
use sitemap_rs::sitemap::Sitemap;

#[test]
fn test_constructor_only_required_fields() {
    let sitemap: Sitemap =
        Sitemap::new(String::from("https://www.toddgriffin.me/sitemap.xml"), None);
    assert!(sitemap.last_modified.is_none());
}

#[test]
fn test_constructor_all_fields() {
    let sitemap: Sitemap = Sitemap::new(String::from(""), Some(DateTime::from(Utc::now())));
    assert!(sitemap.last_modified.is_some());
}
