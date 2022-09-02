use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::sitemap::Sitemap;
use sitemap_rs::sitemap_index::SitemapIndex;
use std::path::PathBuf;

fn main() {
    let sitemaps: Vec<Sitemap> = vec![
        Sitemap::new(
            String::from("http://www.example.com/sitemap1.xml.gz"),
            Some(DateTime::from_utc(
                NaiveDate::from_ymd(2004, 10, 1).and_hms(18, 23, 17),
                FixedOffset::east(0),
            )),
        ),
        Sitemap::new(
            String::from("http://www.example.com/sitemap2.xml.gz"),
            Some(DateTime::from_utc(
                NaiveDate::from_ymd(2005, 1, 1).and_hms(0, 0, 0),
                FixedOffset::east(0),
            )),
        ),
    ];

    let index_sitemap: SitemapIndex =
        SitemapIndex::new(sitemaps).expect("failed a <sitemapindex> validation");
    index_sitemap
        .write_to_file(PathBuf::from("./target/index-sitemap.xml"))
        .unwrap();
}
