extern crate core;

use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::sitemap::Sitemap;
use sitemap_rs::sitemap_index::SitemapIndex;
use sitemap_rs::sitemap_index_error::SitemapIndexError;

#[test]
fn test_write_all_fields() {
    let sitemaps: Vec<Sitemap> = vec![Sitemap::new(
        String::from("https://www.toddgriffin.me/sitemap.xml.gz"),
        Some(DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(1998, 1, 15)
                .unwrap()
                .and_hms_opt(4, 20, 0)
                .unwrap(),
            FixedOffset::east_opt(0).unwrap(),
        )),
    )];
    let index_sitemap: SitemapIndex = SitemapIndex::new(sitemaps).unwrap();

    let mut buf: Vec<u8> = Vec::<u8>::new();
    index_sitemap.write(&mut buf).unwrap();
    let actual: String = String::from_utf8(buf).unwrap();

    let expected: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
	<sitemap>
		<loc>https://www.toddgriffin.me/sitemap.xml.gz</loc>
		<lastmod>1998-01-15T04:20:00+00:00</lastmod>
	</sitemap>
</sitemapindex>
"#;
    assert_eq!(expected, actual);
}

#[test]
fn test_constructor_only_required_fields() {
    let sitemaps: Vec<Sitemap> = vec![Sitemap::new(
        String::from("https://www.toddgriffin.me/sitemap.xml"),
        None,
    )];

    let sitemap_index_result: Result<SitemapIndex, SitemapIndexError> = SitemapIndex::new(sitemaps);
    assert!(sitemap_index_result.is_ok());
}

#[test]
fn test_constructor_too_many_sitemaps() {
    let mut sitemaps: Vec<Sitemap> = vec![];
    for _ in 0..50_001 {
        sitemaps.push(Sitemap::new(
            String::from("https://www.toddgriffin.me/sitemap.xml"),
            None,
        ));
    }

    let sitemap_index_result: Result<SitemapIndex, SitemapIndexError> = SitemapIndex::new(sitemaps);
    match sitemap_index_result {
        Ok(_) => panic!("Returned a SitemapIndex!"),
        Err(e) => match e {
            SitemapIndexError::TooManySitemaps(count) => assert_eq!(50_001, count),
        },
    }
}

#[test]
fn test_write() {
    let sitemaps: Vec<Sitemap> = vec![Sitemap::new(
        String::from("https://www.toddgriffin.me/sitemap.xml"),
        None,
    )];

    let sitemap_index: SitemapIndex = SitemapIndex::new(sitemaps).unwrap();

    let mut buf = Vec::<u8>::new();
    sitemap_index.write(&mut buf).unwrap();
}
