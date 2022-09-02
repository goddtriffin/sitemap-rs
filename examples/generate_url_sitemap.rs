use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::url::{ChangeFrequency, Url};
use sitemap_rs::url_set::UrlSet;
use std::path::PathBuf;

fn main() {
    let url: Url = Url::new(
        String::from("http://www.example.com/"),
        Some(DateTime::from_utc(
            NaiveDate::from_ymd(2005, 1, 1).and_hms(9, 10, 11),
            FixedOffset::east(0),
        )),
        Some(ChangeFrequency::Monthly),
        Some(0.8),
        None,
        None,
        None,
    )
    .expect("failed a <url> validation");

    let url_set: UrlSet = UrlSet::new(vec![url]).expect("failed a <urlset> validation");
    url_set
        .write_to_file(PathBuf::from("./target/sitemap.xml"))
        .unwrap();
}
