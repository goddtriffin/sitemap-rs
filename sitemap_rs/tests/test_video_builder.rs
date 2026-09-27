use jiff::civil::date;
use jiff::tz::{TimeZone, offset};
use sitemap_rs::video::{Platform, PlatformType, Relationship, Restriction, Uploader, Video};
use sitemap_rs::video_builder::VideoBuilder;
use sitemap_rs::video_error::VideoError;
use std::collections::BTreeSet;

#[test]
fn test_only_required_fields() {
    let url_builder_result: Result<Video, VideoError> = VideoBuilder::new(
        String::from("https://www.toddgriffin.me/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        String::from("Alkis shows you how to get perfectly done steaks every time"),
        String::from("https://www.toddgriffin.me/video123.mp4"),
        String::from("https://www.toddgriffin.me/videoplayer.php?video=123"),
    )
    .build();
    assert!(url_builder_result.is_ok());
}

#[test]
fn test_all_fields() {
    let url_builder_result: Result<Video, VideoError> = VideoBuilder::new(
        String::from("https://www.toddgriffin.me/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        String::from("Alkis shows you how to get perfectly done steaks every time"),
        String::from("https://www.toddgriffin.me/video123.mp4"),
        String::from("https://www.toddgriffin.me/videoplayer.php?video=123"),
    )
    .duration(600)
    .expiration_date(
        date(2021, 11, 5)
            .at(19, 20, 30, 0)
            .to_zoned(TimeZone::fixed(offset(8)))
            .unwrap(),
    )
    .rating(4.2)
    .view_count(12345)
    .publication_date(
        date(1998, 1, 15)
            .at(12, 20, 0, 0)
            .to_zoned(TimeZone::fixed(offset(8)))
            .unwrap(),
    )
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
    .build();
    assert!(url_builder_result.is_ok());
}
