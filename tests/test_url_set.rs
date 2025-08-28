use std::collections::BTreeSet;

use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use sitemap_rs::{
    image::Image,
    url::Url,
    video::{PlatformType, Uploader},
};
use sitemap_rs::{
    news::{News, Publication},
    url::{ChangeFrequency, Link},
};
use sitemap_rs::{url_set::UrlSet, video::Video};
use sitemap_rs::{
    url_set_error::UrlSetError,
    video::{Platform, Relationship, Restriction},
};

#[expect(clippy::too_many_lines)]
#[test]
fn test_write_all_fields() {
    let images: Vec<Image> = vec![Image::new(String::from(
        "https://www.toddgriffin.me/picture.webp",
    ))];
    let videos: Vec<Video> = vec![
        Video::builder(
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
        .unwrap(),
    ];
    let news: News = News::new(
        Publication::new(String::from("The Example Times"), String::from("en")),
        DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(1998, 1, 15)
                .unwrap()
                .and_hms_opt(4, 20, 0)
                .unwrap(),
            FixedOffset::east_opt(0).unwrap(),
        ),
        String::from("Companies A, B in Merger Talks"),
    );
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
            .images(images)
            .videos(videos)
            .news(news)
            .build()
            .unwrap(),
    ];
    let url_set: UrlSet = UrlSet::new(urls).unwrap();

    let mut buf: Vec<u8> = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    let actual: String = String::from_utf8(buf).unwrap();

    let expected: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:image="http://www.google.com/schemas/sitemap-image/1.1" xmlns:video="http://www.google.com/schemas/sitemap-video/1.1" xmlns:news="http://www.google.com/schemas/sitemap-news/0.9">
	<url>
		<loc>https://www.toddgriffin.me/</loc>
		<xhtml:link rel="alternate" hreflang="de" href="https://www.toddgriffin.me/de" />
		<lastmod>1998-01-15T04:20:00+00:00</lastmod>
		<changefreq>monthly</changefreq>
		<priority>0.69</priority>
		<image:image>
			<image:loc>https://www.toddgriffin.me/picture.webp</image:loc>
		</image:image>
		<video:video>
			<video:thumbnail_loc>https://www.toddgriffin.me/thumbs/123.jpg</video:thumbnail_loc>
			<video:title>Grilling steaks for summer</video:title>
			<video:description>Alkis shows you how to get perfectly done steaks every time</video:description>
			<video:content_loc>https://www.toddgriffin.me/video123.mp4</video:content_loc>
			<video:player_loc>https://www.toddgriffin.me/videoplayer.php?video=123</video:player_loc>
			<video:duration>600</video:duration>
			<video:expiration_date>2021-11-05T19:20:30+08:00</video:expiration_date>
			<video:rating>4.2</video:rating>
			<video:view_count>8633</video:view_count>
			<video:publication_date>1998-01-15T12:20:00+08:00</video:publication_date>
			<video:family_friendly>yes</video:family_friendly>
			<video:restriction relationship="allow">CA GB IE US</video:restriction>
			<video:platform relationship="allow">tv web</video:platform>
			<video:requires_subscription>yes</video:requires_subscription>
			<video:uploader info="https://www.toddgriffin.me/users/grillymcgrillerson">GrillyMcGrillserson</video:uploader>
			<video:live>no</video:live>
			<video:tag>steak</video:tag>
			<video:tag>meat</video:tag>
			<video:tag>summer</video:tag>
			<video:tag>outdoor</video:tag>
		</video:video>
		<news:news>
			<news:publication>
				<news:name>The Example Times</news:name>
				<news:language>en</news:language>
			</news:publication>
			<news:publication_date>1998-01-15T04:20:00+00:00</news:publication_date>
			<news:title>Companies A, B in Merger Talks</news:title>
		</news:news>
	</url>
</urlset>
"#;
    assert_eq!(expected, actual);
}

#[test]
fn test_constructor_only_required_fields() {
    let urls: Vec<Url> = vec![
        Url::builder(String::from("https://www.toddgriffin.me/"))
            .build()
            .unwrap(),
    ];

    let url_set_result: Result<UrlSet, UrlSetError> = UrlSet::new(urls);
    assert!(url_set_result.is_ok());
}

#[test]
fn test_constructor_too_many_urls() {
    let mut urls: Vec<Url> = vec![];
    for _ in 0..50_001 {
        let url: Url = Url::builder(String::from("https://www.toddgriffin.me/"))
            .build()
            .unwrap();
        urls.push(url);
    }

    let url_set_result: Result<UrlSet, UrlSetError> = UrlSet::new(urls);
    match url_set_result {
        Ok(_) => panic!("Returned a UrlSet!"),
        Err(e) => match e {
            UrlSetError::TooManyUrls(count) => assert_eq!(50_001, count),
            UrlSetError::TooMuchNews(_) => panic!("Returned TooMuchNews!"),
        },
    }
}

#[test]
fn test_constructor_too_much_news() {
    let news: News = News::new(
        Publication::new(String::from("The Todd Times"), String::from("en")),
        DateTime::from(Utc::now()),
        String::from(
            "Local Software Engineer, Todd, Finally Completes Project He Has Talked About For Years",
        ),
    );

    let mut urls: Vec<Url> = vec![];
    for _ in 0..1001 {
        let url: Url = Url::builder(String::from("https://www.toddgriffin.me/"))
            .news(news.clone())
            .build()
            .unwrap();
        urls.push(url);
    }

    let url_set_result: Result<UrlSet, UrlSetError> = UrlSet::new(urls);
    match url_set_result {
        Ok(_) => panic!("Returned a UrlSet!"),
        Err(e) => match e {
            UrlSetError::TooManyUrls(_) => panic!("Returned TooManyUrls!"),
            UrlSetError::TooMuchNews(count) => assert_eq!(1001, count),
        },
    }
}
