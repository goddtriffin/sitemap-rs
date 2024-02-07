use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::news::{News, Publication};
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;

fn main() {
    let urls: Vec<Url> = vec![Url::builder(String::from(
        "http://www.example.org/business/article55.html",
    ))
    .news(News::new(
        Publication::new(String::from("The Example Times"), String::from("en")),
        DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2008, 12, 23)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            FixedOffset::east_opt(0).unwrap(),
        ),
        String::from("Companies A, B in Merger Talks"),
    ))
    .build()
    .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
}
