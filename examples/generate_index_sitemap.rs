use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::sitemap::Sitemap;
use sitemap_rs::sitemap_index::SitemapIndex;

fn main() {
    let sitemaps: Vec<Sitemap> = vec![
        Sitemap::new(
            String::from("https://www.toddgriffin.me/sitemap1.xml.gz"),
            Some(DateTime::from_naive_utc_and_offset(
                NaiveDate::from_ymd_opt(1998, 1, 15)
                    .unwrap()
                    .and_hms_opt(4, 20, 0)
                    .unwrap(),
                FixedOffset::east_opt(0).unwrap(),
            )),
        ),
        Sitemap::new(
            String::from("https://www.toddgriffin.me/sitemap2.xml.gz"),
            Some(DateTime::from_naive_utc_and_offset(
                NaiveDate::from_ymd_opt(2000, 1, 31)
                    .unwrap()
                    .and_hms_opt(4, 20, 0)
                    .unwrap(),
                FixedOffset::east_opt(0).unwrap(),
            )),
        ),
    ];

    let index_sitemap: SitemapIndex = SitemapIndex::new(sitemaps).unwrap();
    let mut buf: Vec<u8> = Vec::<u8>::new();
    index_sitemap.write(&mut buf).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
}
