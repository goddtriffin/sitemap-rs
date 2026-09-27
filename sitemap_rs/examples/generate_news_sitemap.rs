use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::news::{News, Publication};
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;

fn main() {
    let urls: Vec<Url> = vec![
        Url::builder(String::from(
            "https://www.toddgriffin.me/business/article55.html",
        ))
        .news(News::new(
            Publication::new(String::from("The Example Times"), String::from("en")),
            DateTime::from_naive_utc_and_offset(
                NaiveDate::from_ymd_opt(1998, 1, 15)
                    .unwrap()
                    .and_hms_opt(4, 20, 0)
                    .unwrap(),
                FixedOffset::east_opt(0).unwrap(),
            ),
            String::from("Companies A, B in Merger Talks"),
        ))
        .build()
        .unwrap(),
    ];

    let url_set: UrlSet = UrlSet::new(urls).unwrap();
    let mut buf: Vec<u8> = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
}
