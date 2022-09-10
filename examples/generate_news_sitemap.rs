use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::news::{News, Publication};
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;
use std::path::PathBuf;

fn main() {
    let urls: Vec<Url> = vec![Url::builder(String::from(
        "http://www.example.org/business/article55.html",
    ))
    .news(News::new(
        Publication::new(String::from("The Example Times"), String::from("en")),
        DateTime::from_utc(
            NaiveDate::from_ymd(2008, 12, 23).and_hms(0, 0, 0),
            FixedOffset::east(0),
        ),
        String::from("Companies A, B in Merger Talks"),
    ))
    .build()
    .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    url_set
        .write_to_file(PathBuf::from("./target/news-sitemap.xml"))
        .unwrap();
}
