//! Library to generate URL, Index, Image, Video, and News sitemaps.
//!
//! ## Example
//!
//! `cargo run --example generate_url_sitemap`
//!
//! ```rust
//! use chrono::{DateTime, FixedOffset, NaiveDate};
//! use sitemap_rs::url::{ChangeFrequency, Url};
//! use sitemap_rs::url_set::UrlSet;
//! use std::path::PathBuf;
//!
//! let urls: Vec<Url> = vec![Url::new(
//!     String::from("http://www.example.com/"),
//!     Some(DateTime::from_utc(
//!         NaiveDate::from_ymd(2005, 1, 1).and_hms(0, 0, 0),
//!         FixedOffset::east(0),
//!     )),
//!     Some(ChangeFrequency::Monthly),
//!     Some(0.8),
//!     None,
//!     None,
//!     None,
//! )
//! .expect("failed a <url> validation")];
//!
//! let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
//! url_set
//!     .write_to_file(PathBuf::from("./target/url-sitemap.xml"))
//!     .unwrap();
//! ```
//!
//! Generated XML:
//! ```xml
//! <?xml version="1.0" encoding="UTF-8"?>
//! <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
//!   <url>
//!     <loc>http://www.example.com/</loc>
//!     <lastmod>2005-01-01T09:10:11+00:00</lastmod>
//!     <changefreq>monthly</changefreq>
//!     <priority>0.8</priority>
//!   </url>
//! </urlset>
//! ```
//!
//! ### More examples:
//!
//! [Generate URL sitemap](url_set/index.html#examples).
//! [Generate Index sitemap](sitemap_index/index.html#examples).
//! [Generate Image sitemap](image/index.html#examples).
//! [Generate Video sitemap](video/index.html#examples).
//! [Generate News sitemap](news/index.html#examples).

use chrono::SecondsFormat;

pub mod image;
pub mod news;
pub mod sitemap;
pub mod sitemap_index;
pub mod sitemap_index_error;
pub mod url;
pub mod url_error;
pub mod url_set;
pub mod url_set_error;
pub mod video;
pub mod video_error;

pub const NAMESPACE: &str = "http://www.sitemaps.org/schemas/sitemap/0.9";
pub const IMAGE_NAMESPACE: &str = "http://www.google.com/schemas/sitemap-image/1.1";
pub const VIDEO_NAMESPACE: &str = "http://www.google.com/schemas/sitemap-video/1.1";
pub const NEWS_NAMESPACE: &str = "http://www.google.com/schemas/sitemap-news/0.9";
pub const ENCODING: &str = "UTF-8";
pub const RFC_3339_SECONDS_FORMAT: SecondsFormat = SecondsFormat::Secs;
pub const RFC_3339_USE_Z: bool = false;
