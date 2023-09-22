use chrono::DateTime;
use pretty_assertions::assert_eq;
use sitemap_rs::sitemap::Sitemap;
use sitemap_rs::sitemap_index::SitemapIndex;

#[test]
fn test_index_sitemap_generation() {
    let sitemaps: Vec<Sitemap> = vec![
        Sitemap::new(
            String::from("https://www.example.com/sitemap1.xml.gz"),
            Some(DateTime::parse_from_rfc3339("2023-01-01T13:37:00+00:00").unwrap()),
        ),
        Sitemap::new(
            String::from("https://www.example.com/sitemap2.xml.gz"),
            Some(DateTime::parse_from_rfc3339("2023-03-03T08:15:00+00:00").unwrap()),
        ),
    ];

    let index_sitemap: SitemapIndex =
        SitemapIndex::new(sitemaps).expect("failed a <sitemapindex> validation");
    let mut buf = Vec::<u8>::new();
    index_sitemap.write(&mut buf).unwrap();
    let result = String::from_utf8(buf).unwrap();
    assert_eq!(
        "\
        <?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <sitemapindex xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n\
            \t<sitemap>\n\
                \t\t<loc>https://www.example.com/sitemap1.xml.gz</loc>\n\
                \t\t<lastmod>2023-01-01T13:37:00+00:00</lastmod>\n\
            \t</sitemap>\n\
            \t<sitemap>\n\
                \t\t<loc>https://www.example.com/sitemap2.xml.gz</loc>\n\
                \t\t<lastmod>2023-03-03T08:15:00+00:00</lastmod>\n\
            \t</sitemap>\n\
        </sitemapindex>\n",
        result
    );
}
