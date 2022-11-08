use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::url::{ChangeFrequency, Url};
use sitemap_rs::url_set::UrlSet;

fn main() {
    let urls: Vec<Url> = vec![Url::builder(String::from("http://www.example.com/"))
        .last_modified(DateTime::from_utc(
            NaiveDate::from_ymd(2005, 1, 1).and_hms(0, 0, 0),
            FixedOffset::east(0),
        ))
        .change_frequency(ChangeFrequency::Monthly)
        .priority(0.8)
        .build()
        .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
}
