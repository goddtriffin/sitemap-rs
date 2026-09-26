//! Deterministic fixtures and scenarios shared by every benchmark binary.
//!
//! Only API that is independent of the underlying XML library is used here (builders, `new()`,
//! `write()`, and `.unwrap()`), so this code compiles unchanged across XML library migrations.

use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::image::Image;
use sitemap_rs::news::{News, Publication};
use sitemap_rs::sitemap::Sitemap;
use sitemap_rs::sitemap_index::SitemapIndex;
use sitemap_rs::url::{ChangeFrequency, Link, Url};
use sitemap_rs::url_set::UrlSet;
use sitemap_rs::video::{Platform, PlatformType, Relationship, Restriction, Uploader, Video};
use std::collections::BTreeSet;

/// The maximum number of URLs allowed in a `UrlSet`.
const MAX_URLS: usize = 50_000;

/// The maximum number of URLs allowed in a `UrlSet` that contains news.
const MAX_NEWS_URLS: usize = 1_000;

/// The maximum number of sitemaps allowed in a `SitemapIndex`.
const MAX_SITEMAPS: usize = 50_000;

/// A benchmark scenario: prepared input data, and the user-facing operation to measure.
#[derive(Clone)]
pub enum Scenario {
    UrlSet(Vec<Url>),
    SitemapIndex(Vec<Sitemap>),
}

impl Scenario {
    /// Every scenario in the suite, as `(name, scenario)`.
    pub fn all() -> Vec<(&'static str, Self)> {
        vec![
            ("url_set/plain_50k", Self::UrlSet(plain_urls(MAX_URLS))),
            ("url_set/rich_1k", Self::UrlSet(rich_urls(MAX_NEWS_URLS))),
            (
                "sitemap_index/50k",
                Self::SitemapIndex(sitemaps(MAX_SITEMAPS)),
            ),
        ]
    }

    /// Runs the measured operation: construct the container, then write it as XML into `buf`.
    pub fn run(self, buf: &mut Vec<u8>) {
        match self {
            Self::UrlSet(urls) => UrlSet::new(urls).unwrap().write(buf).unwrap(),
            Self::SitemapIndex(sitemaps) => {
                SitemapIndex::new(sitemaps).unwrap().write(buf).unwrap();
            }
        }
    }
}

fn date(year: i32, month: u32, day: u32) -> DateTime<FixedOffset> {
    DateTime::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(4, 20, 0)
            .unwrap(),
        FixedOffset::east_opt(0).unwrap(),
    )
}

/// The most common sitemap shape: `<loc>`, `<lastmod>`, `<changefreq>`, and `<priority>`.
fn plain_urls(count: usize) -> Vec<Url> {
    (0..count)
        .map(|i| {
            Url::builder(format!("https://www.example.com/page/{i}"))
                .last_modified(date(2024, 1, 15))
                .change_frequency(ChangeFrequency::Weekly)
                .priority(0.5)
                .build()
                .unwrap()
        })
        .collect()
}

/// Exercises every element type: hreflang links, images, videos, and news.
fn rich_urls(count: usize) -> Vec<Url> {
    (0..count)
        .map(|i| {
            Url::builder(format!("https://www.example.com/article/{i}"))
                .links(vec![
                    Link::new(
                        "de".to_owned(),
                        format!("https://www.example.com/de/article/{i}"),
                    ),
                    Link::new(
                        "fr".to_owned(),
                        format!("https://www.example.com/fr/article/{i}"),
                    ),
                ])
                .last_modified(date(2024, 1, 15))
                .change_frequency(ChangeFrequency::Daily)
                .priority(0.8)
                .images(vec![
                    Image::new(format!("https://www.example.com/images/{i}/1.jpg")),
                    Image::new(format!("https://www.example.com/images/{i}/2.jpg")),
                ])
                .videos(vec![video(i)])
                .news(News::new(
                    Publication::new(String::from("The Example Times"), String::from("en")),
                    date(2024, 1, 15),
                    format!("Article number {i} & friends <breaking>"),
                ))
                .build()
                .unwrap()
        })
        .collect()
}

fn video(i: usize) -> Video {
    Video::builder(
        format!("https://www.example.com/thumbs/{i}.jpg"),
        format!("Video number {i}"),
        String::from("A description of the video, long enough to be realistic but not huge."),
        format!("https://www.example.com/videos/{i}.mp4"),
        format!("https://www.example.com/player?video={i}"),
    )
    .duration(600)
    .expiration_date(date(2030, 1, 1))
    .rating(4.2)
    .view_count(8633)
    .publication_date(date(2024, 1, 15))
    .family_friendly(true)
    .restriction(Restriction::new(
        BTreeSet::from([String::from("CA"), String::from("GB"), String::from("US")]),
        Relationship::Allow,
    ))
    .platform(Platform::new(
        BTreeSet::from([PlatformType::Web, PlatformType::Tv]),
        Relationship::Allow,
    ))
    .requires_subscription(false)
    .uploader(Uploader::new(
        String::from("ExampleUploader"),
        Some(String::from("https://www.example.com/users/example")),
    ))
    .live(false)
    .tags(vec![String::from("steak"), String::from("summer")])
    .build()
    .unwrap()
}

fn sitemaps(count: usize) -> Vec<Sitemap> {
    (0..count)
        .map(|i| {
            Sitemap::new(
                format!("https://www.example.com/sitemaps/sitemap-{i}.xml.gz"),
                Some(date(2024, 1, 15)),
            )
        })
        .collect()
}
