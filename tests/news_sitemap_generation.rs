use chrono::DateTime;
use pretty_assertions::assert_eq;
use sitemap_rs::news::{News, Publication};
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;

#[test]
fn test_news_sitemap_generation() {
    let urls: Vec<Url> = vec![Url::builder(String::from("https://www.example.com/"))
        .news(News::new(
            Publication::new(String::from("News Site"), String::from("en")),
            DateTime::parse_from_rfc3339("2023-01-01T13:37:00+00:00").unwrap(),
            String::from("Awesome Title of News Article"),
        ))
        .build()
        .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    let result = String::from_utf8(buf).unwrap();
    assert_eq!(
        "\
        <?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" xmlns:news=\"http://www.google.com/schemas/sitemap-news/0.9\">\n\
            \t<url>\n\
                \t\t<loc>https://www.example.com/</loc>\n\
                \t\t<news:news>\n\
                    \t\t\t<news:publication>\n\
                        \t\t\t\t<news:name>News Site</news:name>\n\
                        \t\t\t\t<news:language>en</news:language>\n\
                    \t\t\t</news:publication>\n\
                    \t\t\t<news:publication_date>2023-01-01T13:37:00+00:00</news:publication_date>\n\
                    \t\t\t<news:title>Awesome Title of News Article</news:title>\n\
                \t\t</news:news>\n\
            \t</url>\n\
        </urlset>\n",
        result
    );
}
