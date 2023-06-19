use chrono::DateTime;
use pretty_assertions::assert_eq;
use sitemap_rs::url::{Alternate, ChangeFrequency, Url, DEFAULT_PRIORITY};
use sitemap_rs::url_set::UrlSet;

#[test]
fn test_sitemap_generation_location() {
    let urls: Vec<Url> = vec![Url::builder(String::from("https://www.example.com/"))
        .build()
        .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    let result = String::from_utf8(buf).unwrap();
    assert_eq!(
        "\
        <?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n\
            \t<url>\n\
                \t\t<loc>https://www.example.com/</loc>\n\
            \t</url>\n\
        </urlset>\n",
        result
    );
}

#[test]
fn test_sitemap_generation_last_modified() {
    let urls: Vec<Url> = vec![Url::builder(String::from("https://www.example.com/"))
        .last_modified(DateTime::parse_from_rfc3339("2023-01-01T13:37:00+00:00").unwrap())
        .build()
        .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    let result = String::from_utf8(buf).unwrap();
    assert_eq!(
        "\
        <?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n\
            \t<url>\n\
                \t\t<loc>https://www.example.com/</loc>\n\
                \t\t<lastmod>2023-01-01T13:37:00+00:00</lastmod>\n\
            \t</url>\n\
        </urlset>\n",
        result
    );
}

#[test]
fn test_sitemap_generation_change_frequency() {
    let urls: Vec<Url> = vec![Url::builder(String::from("https://www.example.com/"))
        .change_frequency(ChangeFrequency::Weekly)
        .build()
        .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    let result = String::from_utf8(buf).unwrap();
    assert_eq!(
        "\
        <?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n\
            \t<url>\n\
                \t\t<loc>https://www.example.com/</loc>\n\
                \t\t<changefreq>weekly</changefreq>\n\
            \t</url>\n\
        </urlset>\n",
        result
    );
}

#[test]
fn test_sitemap_generation_priority() {
    let urls: Vec<Url> = vec![Url::builder(String::from("https://www.example.com/"))
        .priority(DEFAULT_PRIORITY)
        .build()
        .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    let result = String::from_utf8(buf).unwrap();
    assert_eq!(
        "\
        <?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n\
            \t<url>\n\
                \t\t<loc>https://www.example.com/</loc>\n\
                \t\t<priority>0.5</priority>\n\
            \t</url>\n\
        </urlset>\n",
        result
    );
}

#[test]
fn test_sitemap_generation_alternates() {
    let urls: Vec<Url> = vec![Url::builder(String::from("https://www.example.com/"))
        .alternates(vec![
            Alternate {
                hreflang: String::from("en-US"),
                href: String::from("https://www.example.com/"),
            },
            Alternate {
                hreflang: String::from("de-DE"),
                href: String::from("https://de.example.com/"),
            },
        ])
        .push_alternate(
            String::from("x-default"),
            String::from("https://www.example.com/country-selector"),
        )
        .push_alternate(String::from("it"), String::from("https://it.example.com/"))
        .build()
        .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    let result = String::from_utf8(buf).unwrap();
    assert_eq!(
        "\
        <?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" xmlns:xhtml=\"http://www.w3.org/1999/xhtml\">\n\
            \t<url>\n\
                \t\t<loc>https://www.example.com/</loc>\n\
                \t\t<xhtml:link rel=\"alternate\" hreflang=\"en-US\" href=\"https://www.example.com/\" />\n\
                \t\t<xhtml:link rel=\"alternate\" hreflang=\"de-DE\" href=\"https://de.example.com/\" />\n\
                \t\t<xhtml:link rel=\"alternate\" hreflang=\"x-default\" href=\"https://www.example.com/country-selector\" />\n\
                \t\t<xhtml:link rel=\"alternate\" hreflang=\"it\" href=\"https://it.example.com/\" />\n\
            \t</url>\n\
        </urlset>\n",
        result
    );
}

#[test]
fn test_sitemap_generation_all() {
    let urls: Vec<Url> = vec![Url::builder(String::from("https://www.example.com/"))
        .last_modified(DateTime::parse_from_rfc3339("2023-01-01T13:37:00+00:00").unwrap())
        .change_frequency(ChangeFrequency::Weekly)
        .priority(DEFAULT_PRIORITY)
        .alternates(vec![
            Alternate {
                hreflang: String::from("en-US"),
                href: String::from("https://www.example.com/"),
            },
            Alternate {
                hreflang: String::from("de-DE"),
                href: String::from("https://de.example.com/"),
            },
        ])
        .push_alternate(
            String::from("x-default"),
            String::from("https://www.example.com/country-selector"),
        )
        .push_alternate(String::from("it"), String::from("https://it.example.com/"))
        .build()
        .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    let result = String::from_utf8(buf).unwrap();
    assert_eq!(
        "\
        <?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" xmlns:xhtml=\"http://www.w3.org/1999/xhtml\">\n\
            \t<url>\n\
                \t\t<loc>https://www.example.com/</loc>\n\
                \t\t<lastmod>2023-01-01T13:37:00+00:00</lastmod>\n\
                \t\t<changefreq>weekly</changefreq>\n\
                \t\t<priority>0.5</priority>\n\
                \t\t<xhtml:link rel=\"alternate\" hreflang=\"en-US\" href=\"https://www.example.com/\" />\n\
                \t\t<xhtml:link rel=\"alternate\" hreflang=\"de-DE\" href=\"https://de.example.com/\" />\n\
                \t\t<xhtml:link rel=\"alternate\" hreflang=\"x-default\" href=\"https://www.example.com/country-selector\" />\n\
                \t\t<xhtml:link rel=\"alternate\" hreflang=\"it\" href=\"https://it.example.com/\" />\n\
            \t</url>\n\
        </urlset>\n",
        result
    );
}
