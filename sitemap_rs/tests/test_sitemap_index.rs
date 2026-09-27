extern crate core;

use jiff::Zoned;
use jiff::civil::date;
use jiff::tz::{Offset, TimeZone, offset};
use sitemap_rs::sitemap::Sitemap;
use sitemap_rs::sitemap_index::SitemapIndex;
use sitemap_rs::sitemap_index_error::SitemapIndexError;

#[test]
fn test_write_all_fields() {
    let sitemaps: Vec<Sitemap> = vec![Sitemap::new(
        String::from("https://www.toddgriffin.me/sitemap.xml.gz"),
        Some(
            date(1998, 1, 15)
                .at(4, 20, 0, 0)
                .to_zoned(TimeZone::UTC)
                .unwrap(),
        ),
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
fn test_write_escapes_special_characters() {
    let sitemaps: Vec<Sitemap> = vec![Sitemap::new(
        String::from("https://www.toddgriffin.me/sitemap.xml?page=1&lang=en"),
        None,
    )];
    let index_sitemap: SitemapIndex = SitemapIndex::new(sitemaps).unwrap();

    let mut buf: Vec<u8> = Vec::<u8>::new();
    index_sitemap.write(&mut buf).unwrap();
    let actual: String = String::from_utf8(buf).unwrap();

    let expected: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
	<sitemap>
		<loc>https://www.toddgriffin.me/sitemap.xml?page=1&amp;lang=en</loc>
	</sitemap>
</sitemapindex>
"#;
    assert_eq!(expected, actual);
}

#[test]
fn test_write_empty() {
    let index_sitemap: SitemapIndex = SitemapIndex::new(vec![]).unwrap();

    let mut buf: Vec<u8> = Vec::<u8>::new();
    index_sitemap.write(&mut buf).unwrap();
    let actual: String = String::from_utf8(buf).unwrap();

    let expected: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" />
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

/// Writes a `SitemapIndex` with a single `<lastmod>` and returns the `<lastmod>` text.
fn written_last_modified(last_modified: Zoned) -> String {
    let sitemaps: Vec<Sitemap> = vec![Sitemap::new(
        String::from("https://www.toddgriffin.me/sitemap.xml"),
        Some(last_modified),
    )];
    let mut buf: Vec<u8> = Vec::<u8>::new();
    SitemapIndex::new(sitemaps)
        .unwrap()
        .write(&mut buf)
        .unwrap();
    let xml: String = String::from_utf8(buf).unwrap();

    let start: usize = xml.find("<lastmod>").unwrap() + "<lastmod>".len();
    let end: usize = xml.find("</lastmod>").unwrap();
    xml[start..end].to_owned()
}

#[test]
fn test_write_date_truncates_fractional_seconds() {
    let last_modified: Zoned = date(1998, 1, 15)
        .at(4, 20, 0, 999_999_999)
        .to_zoned(TimeZone::UTC)
        .unwrap();
    assert_eq!(
        "1998-01-15T04:20:00+00:00",
        written_last_modified(last_modified)
    );
}

#[test]
fn test_write_date_negative_offset() {
    let last_modified: Zoned = date(1998, 1, 15)
        .at(4, 20, 0, 0)
        .to_zoned(TimeZone::fixed(offset(-5)))
        .unwrap();
    assert_eq!(
        "1998-01-15T04:20:00-05:00",
        written_last_modified(last_modified)
    );
}

#[test]
fn test_write_date_non_hour_offset() {
    let last_modified: Zoned = date(1998, 1, 15)
        .at(4, 20, 0, 0)
        .to_zoned(TimeZone::fixed(
            Offset::from_seconds(5 * 3600 + 30 * 60).unwrap(),
        ))
        .unwrap();
    assert_eq!(
        "1998-01-15T04:20:00+05:30",
        written_last_modified(last_modified)
    );
}

#[test]
fn test_write_date_iana_time_zone() {
    let last_modified: Zoned = date(2024, 7, 4)
        .at(12, 0, 0, 0)
        .in_tz("America/New_York")
        .unwrap();
    assert_eq!(
        "2024-07-04T12:00:00-04:00",
        written_last_modified(last_modified)
    );
}
