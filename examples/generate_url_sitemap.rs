use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::url::{ChangeFrequency, Url};
use sitemap_rs::url_set::UrlSet;

fn main() {
    let urls: Vec<Url> = vec![
        Url::builder(String::from("http://www.example.com/"))
            .last_modified(DateTime::from_naive_utc_and_offset(
                NaiveDate::from_ymd_opt(2005, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                FixedOffset::east_opt(0).unwrap(),
            ))
            .change_frequency(ChangeFrequency::Monthly)
            .priority(0.8)
            .build()
            .expect("failed a <url> validation"),
    ];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
}
