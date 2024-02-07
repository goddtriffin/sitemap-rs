use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::sitemap::Sitemap;
use sitemap_rs::sitemap_index::SitemapIndex;

fn main() {
    let sitemaps: Vec<Sitemap> = vec![
        Sitemap::new(
            String::from("http://www.example.com/sitemap1.xml.gz"),
            Some(DateTime::from_naive_utc_and_offset(
                NaiveDate::from_ymd_opt(2004, 10, 1)
                    .unwrap()
                    .and_hms_opt(18, 23, 17)
                    .unwrap(),
                FixedOffset::east_opt(0).unwrap(),
            )),
        ),
        Sitemap::new(
            String::from("http://www.example.com/sitemap2.xml.gz"),
            Some(DateTime::from_naive_utc_and_offset(
                NaiveDate::from_ymd_opt(2005, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                FixedOffset::east_opt(0).unwrap(),
            )),
        ),
    ];

    let index_sitemap: SitemapIndex =
        SitemapIndex::new(sitemaps).expect("failed a <sitemapindex> validation");
    let mut buf = Vec::<u8>::new();
    index_sitemap.write(&mut buf).unwrap();
}
