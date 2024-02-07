use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::video::{Platform, PlatformType, Relationship, Restriction, Uploader, Video};
use sitemap_rs::video_builder::VideoBuilder;
use sitemap_rs::video_error::VideoError;
use std::collections::HashSet;

#[test]
fn test_only_required_fields() {
    let url_builder_result: Result<Video, VideoError> = VideoBuilder::new(
        String::from("http://www.example.com/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        String::from("Alkis shows you how to get perfectly done steaks every time"),
        String::from("http://streamserver.example.com/video123.mp4"),
        String::from("http://www.example.com/videoplayer.php?video=123"),
    )
    .build();
    assert!(url_builder_result.is_ok());
}

#[test]
fn test_all_fields() {
    let url_builder_result: Result<Video, VideoError> = VideoBuilder::new(
        String::from("http://www.example.com/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        String::from("Alkis shows you how to get perfectly done steaks every time"),
        String::from("http://streamserver.example.com/video123.mp4"),
        String::from("http://www.example.com/videoplayer.php?video=123"),
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
    .view_count(12345)
    .publication_date(DateTime::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(2007, 11, 5)
            .unwrap()
            .and_hms_opt(11, 20, 30)
            .unwrap(),
        FixedOffset::east_opt(8 * 3600).unwrap(),
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
    .build();
    assert!(url_builder_result.is_ok());
}
