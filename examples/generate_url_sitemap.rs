use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::url::{ChangeFrequency, Link, Url};
use sitemap_rs::url_set::UrlSet;

fn main() {
    let urls: Vec<Url> = vec![
        Url::builder(String::from("https://www.toddgriffin.me/"))
            .links(vec![Link::new(
                "de".to_owned(),
                "https://www.toddgriffin.me/de".to_owned(),
            )])
            .last_modified(DateTime::from_naive_utc_and_offset(
                NaiveDate::from_ymd_opt(1998, 1, 15)
                    .unwrap()
                    .and_hms_opt(4, 20, 0)
                    .unwrap(),
                FixedOffset::east_opt(0).unwrap(),
            ))
            .change_frequency(ChangeFrequency::Monthly)
            .priority(0.69)
            .build()
            .unwrap(),
    ];

    let url_set: UrlSet = UrlSet::new(urls).unwrap();
    let mut buf: Vec<u8> = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
}
