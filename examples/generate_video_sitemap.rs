use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;
use sitemap_rs::video::{Platform, PlatformType, Relationship, Restriction, Uploader, Video};
use std::collections::HashSet;
use std::path::PathBuf;

fn main() {
    let urls: Vec<Url> = vec![Url::new(
        String::from("http://www.example.com/videos/some_video_landing_page.html"),
        None,
        None,
        None,
        None,
        Some(vec![Video::new(
            String::from("http://www.example.com/thumbs/123.jpg"),
            String::from("Grilling steaks for summer"),
            String::from("Alkis shows you how to get perfectly done steaks every time"),
            String::from("http://streamserver.example.com/video123.mp4"),
            String::from("http://www.example.com/videoplayer.php?video=123"),
            Some(600),
            Some(DateTime::from_utc(
                NaiveDate::from_ymd(2021, 11, 5).and_hms(11, 20, 30),
                FixedOffset::east(8 * 3600),
            )),
            Some(4.2),
            Some(12345),
            Some(DateTime::from_utc(
                NaiveDate::from_ymd(2007, 11, 5).and_hms(11, 20, 30),
                FixedOffset::east(8 * 3600),
            )),
            Some(true),
            Some(Restriction::new(
                HashSet::from([
                    String::from("IE"),
                    String::from("GB"),
                    String::from("US"),
                    String::from("CA"),
                ]),
                Relationship::Allow,
            )),
            Some(Platform::new(
                HashSet::from([PlatformType::Web, PlatformType::Tv]),
                Relationship::Allow,
            )),
            Some(true),
            Some(Uploader::new(
                String::from("GrillyMcGrillserson"),
                Some(String::from(
                    "http://www.example.com/users/grillymcgrillerson",
                )),
            )),
            Some(false),
            Some(vec![
                String::from("steak"),
                String::from("meat"),
                String::from("summer"),
                String::from("outdoor"),
            ]),
        )
        .expect("failed a <video:video> validation")]),
        None,
    )
    .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    url_set
        .write_to_file(PathBuf::from("./target/video-sitemap.xml"))
        .unwrap();
}
