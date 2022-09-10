use chrono::{DateTime, Utc};
use sitemap_rs::image::Image;
use sitemap_rs::news::{News, Publication};
use sitemap_rs::url::{ChangeFrequency, Url, DEFAULT_PRIORITY};
use sitemap_rs::url_builder::UrlBuilder;
use sitemap_rs::url_error::UrlError;
use sitemap_rs::video::Video;

#[test]
fn test_only_required_fields() {
    let url_builder_result: Result<Url, UrlError> =
        UrlBuilder::new(String::from("https://www.toddgriffin.me/")).build();
    assert!(url_builder_result.is_ok());
}

#[test]
fn test_all_normal_fields() {
    let url_builder_result: Result<Url, UrlError> =
        UrlBuilder::new(String::from("https://www.toddgriffin.me/"))
            .last_modified(DateTime::from(Utc::now()))
            .change_frequency(ChangeFrequency::Weekly)
            .priority(DEFAULT_PRIORITY)
            .build();
    assert!(url_builder_result.is_ok());
}

#[test]
fn test_all_fields() {
    let url_builder_result: Result<Url, UrlError> =
        UrlBuilder::new(String::from("https://www.toddgriffin.me/"))
            .last_modified(DateTime::from(Utc::now()))
            .change_frequency(ChangeFrequency::Weekly)
            .priority(DEFAULT_PRIORITY)
            .images(vec![Image::new(String::from("https://www.toddgriffin.me/static/image/social/profile-picture.webp"))])
            .videos(vec![Video::new(
                String::from("http://www.example.com/thumbs/123.jpg"),
                String::from("Grilling steaks for summer"),
                String::from("Alkis shows you how to get perfectly done steaks every time"),
                String::from("http://streamserver.example.com/video123.mp4"),
                String::from("http://www.example.com/videoplayer.php?video=123"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ).unwrap()])
            .news(News::new(
                Publication::new(String::from("The Todd Times"), String::from("en")),
                DateTime::from(Utc::now()),
                String::from(
                    "Local Software Engineer, Todd, Finally Completes Project He Has Talked About For Years",
                ),
            ))
            .build();
    assert!(url_builder_result.is_ok());
}
