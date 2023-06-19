use chrono::DateTime;
use pretty_assertions::assert_eq;
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;
use sitemap_rs::video::{Uploader, Video};

#[test]
fn test_video_sitemap_generation() {
    let video: Video = Video::builder(
        String::from("https://www.example.com/thumbnail.jpg"),
        String::from("Video Title"),
        String::from("Video description"),
        String::from("https://www.example.com/content_location.mp4"),
        String::from("https://www.example.com/player_location.php"),
    )
    .duration(600)
    .expiration_date(DateTime::parse_from_rfc3339("2025-01-01T13:37:00+00:00").unwrap())
    .rating(4.2)
    .view_count(12345)
    .publication_date(DateTime::parse_from_rfc3339("2009-01-01T13:37:00+00:00").unwrap())
    .family_friendly(true)
    /*
    XXX: Add restriction and platform in test. Those are built from HashSets
    and therefore have no order, which makes assertions on equality difficult.
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
     */
    .requires_subscription(true)
    .uploader(Uploader::new(
        String::from("UserName"),
        Some(String::from("https://www.example.com/users/UserName")),
    ))
    .live(false)
    .tags(vec![
        String::from("video tag 1"),
        String::from("video tag 2"),
    ])
    .build()
    .expect("failed a <video:video> validation");

    let urls: Vec<Url> = vec![Url::builder(String::from(
        "https://www.example.com/videos/some_video_landing_page.html",
    ))
    .videos(vec![video])
    .build()
    .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    let result = String::from_utf8(buf).unwrap();
    assert_eq!(
        "\
        <?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" xmlns:video=\"http://www.google.com/schemas/sitemap-video/1.1\">\n\
            \t<url>\n\
                \t\t<loc>https://www.example.com/videos/some_video_landing_page.html</loc>\n\
                \t\t<video:video>\n\
                    \t\t\t<video:thumbnail_loc>https://www.example.com/thumbnail.jpg</video:thumbnail_loc>\n\
                    \t\t\t<video:title>Video Title</video:title>\n\
                    \t\t\t<video:description>Video description</video:description>\n\
                    \t\t\t<video:content_loc>https://www.example.com/content_location.mp4</video:content_loc>\n\
                    \t\t\t<video:player_loc>https://www.example.com/player_location.php</video:player_loc>\n\
                    \t\t\t<video:duration>600</video:duration>\n\
                    \t\t\t<video:expiration_date>2025-01-01T13:37:00+00:00</video:expiration_date>\n\
                    \t\t\t<video:rating>4.2</video:rating>\n\
                    \t\t\t<video:view_count>12345</video:view_count>\n\
                    \t\t\t<video:publication_date>2009-01-01T13:37:00+00:00</video:publication_date>\n\
                    \t\t\t<video:family_friendly>yes</video:family_friendly>\n\
                    \t\t\t<video:requires_subscription>yes</video:requires_subscription>\n\
                    \t\t\t<video:uploader info=\"https://www.example.com/users/UserName\">UserName</video:uploader>\n\
                    \t\t\t<video:live>no</video:live>\n\
                    \t\t\t<video:tag>video tag 1</video:tag>\n\
                    \t\t\t<video:tag>video tag 2</video:tag>\n\
                \t\t</video:video>\n\
            \t</url>\n\
        </urlset>\n",
        result
    );
}
