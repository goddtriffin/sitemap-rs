use jiff::civil::date;
use jiff::tz::TimeZone;
use sitemap_rs::sitemap::Sitemap;
use sitemap_rs::sitemap_index::SitemapIndex;

fn main() {
    let sitemaps: Vec<Sitemap> = vec![
        Sitemap::new(
            String::from("https://www.toddgriffin.me/sitemap1.xml.gz"),
            Some(
                date(1998, 1, 15)
                    .at(4, 20, 0, 0)
                    .to_zoned(TimeZone::UTC)
                    .unwrap(),
            ),
        ),
        Sitemap::new(
            String::from("https://www.toddgriffin.me/sitemap2.xml.gz"),
            Some(
                date(2000, 1, 31)
                    .at(4, 20, 0, 0)
                    .to_zoned(TimeZone::UTC)
                    .unwrap(),
            ),
        ),
    ];

    let index_sitemap: SitemapIndex = SitemapIndex::new(sitemaps).unwrap();
    let mut buf: Vec<u8> = Vec::<u8>::new();
    index_sitemap.write(&mut buf).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
}
