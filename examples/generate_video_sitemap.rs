use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;
use sitemap_rs::video::{Platform, PlatformType, Relationship, Restriction, Uploader, Video};
use std::collections::BTreeSet;

fn main() {
    let video: Video = Video::builder(
        String::from("https://www.toddgriffin.me/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        String::from("Alkis shows you how to get perfectly done steaks every time"),
        String::from("https://www.toddgriffin.me/video123.mp4"),
        String::from("https://www.toddgriffin.me/videoplayer.php?video=123"),
    )
    .duration(600)
    .expiration_date(DateTime::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(2021, 11, 5)
            .unwrap()
            .and_hms_opt(11, 20, 30)
            .unwrap(),
        FixedOffset::east_opt(8 * 3600).unwrap(),
    ))
    .rating(4.2)
    .view_count(8633)
    .publication_date(DateTime::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(1998, 1, 15)
            .unwrap()
            .and_hms_opt(4, 20, 0)
            .unwrap(),
        FixedOffset::east_opt(8 * 3600).unwrap(),
    ))
    .family_friendly(true)
    .restriction(Restriction::new(
        BTreeSet::from([
            String::from("IE"),
            String::from("GB"),
            String::from("US"),
            String::from("CA"),
        ]),
        Relationship::Allow,
    ))
    .platform(Platform::new(
        BTreeSet::from([PlatformType::Web, PlatformType::Tv]),
        Relationship::Allow,
    ))
    .requires_subscription(true)
    .uploader(Uploader::new(
        String::from("GrillyMcGrillserson"),
        Some(String::from(
            "https://www.toddgriffin.me/users/grillymcgrillerson",
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
    .unwrap();

    let urls: Vec<Url> = vec![
        Url::builder(String::from(
            "https://www.toddgriffin.me/videos/some_video_landing_page.html",
        ))
        .videos(vec![video])
        .build()
        .unwrap(),
    ];

    let url_set: UrlSet = UrlSet::new(urls).unwrap();
    let mut buf: Vec<u8> = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
}
