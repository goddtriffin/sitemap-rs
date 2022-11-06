use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;
use sitemap_rs::video::{Platform, PlatformType, Relationship, Restriction, Uploader, Video};
use std::collections::HashSet;

fn main() {
    let video: Video = Video::builder(
        String::from("http://www.example.com/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        String::from("Alkis shows you how to get perfectly done steaks every time"),
        String::from("http://streamserver.example.com/video123.mp4"),
        String::from("http://www.example.com/videoplayer.php?video=123"),
    )
    .duration(600)
    .expiration_date(DateTime::from_utc(
        NaiveDate::from_ymd(2021, 11, 5).and_hms(11, 20, 30),
        FixedOffset::east(8 * 3600),
    ))
    .rating(4.2)
    .view_count(12345)
    .publication_date(DateTime::from_utc(
        NaiveDate::from_ymd(2007, 11, 5).and_hms(11, 20, 30),
        FixedOffset::east(8 * 3600),
    ))
    .family_friendly(true)
    .restriction(Restriction::new(
        HashSet::from([
            String::from("IE"),
            String::from("GB"),
            String::from("US"),
            String::from("CA"),
        ]),
        Relationship::Allow,
    ))
    .platform(Platform::new(
        HashSet::from([PlatformType::Web, PlatformType::Tv]),
        Relationship::Allow,
    ))
    .requires_subscription(true)
    .uploader(Uploader::new(
        String::from("GrillyMcGrillserson"),
        Some(String::from(
            "http://www.example.com/users/grillymcgrillerson",
        )),
    ))
    .live(false)
    .tags(vec![
        String::from("steak"),
        String::from("meat"),
        String::from("summer"),
        String::from("outdoor"),
    ])
    .build()
    .expect("failed a <video:video> validation");

    let urls: Vec<Url> = vec![Url::builder(String::from(
        "http://www.example.com/videos/some_video_landing_page.html",
    ))
    .videos(vec![video])
    .build()
    .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
}
