//! A Rust library to generate URL, Index, Image, Video, and News sitemaps.
//!
//! ## Example
//!
//! `cargo run --example generate_url_sitemap`
//!
//! ```rust
//! use jiff::civil::date;
//! use jiff::tz::TimeZone;
//! use sitemap_rs::url::{ChangeFrequency, Link, Url};
//! use sitemap_rs::url_set::UrlSet;
//!
//! let urls: Vec<Url> = vec![
//!     Url::builder(String::from("https://www.toddgriffin.me/"))
//!         .links(vec![Link::new(
//!             "de".to_owned(),
//!             "https://www.toddgriffin.me/de".to_owned(),
//!         )])
//!         .last_modified(
//!             date(1998, 1, 15)
//!                 .at(4, 20, 0, 0)
//!                 .to_zoned(TimeZone::UTC)
//!                 .unwrap(),
//!         )
//!         .change_frequency(ChangeFrequency::Monthly)
//!         .priority(0.69)
//!         .build()
//!         .unwrap(),
//! ];
//!
//! let url_set: UrlSet = UrlSet::new(urls).unwrap();
//! let mut buf: Vec<u8> = Vec::<u8>::new();
//! url_set.write(&mut buf).unwrap();
//! println!("{}", String::from_utf8(buf).unwrap());
//! ```
//!
//! Generated XML:
//! ```xml
//! <?xml version="1.0" encoding="UTF-8"?>
//! <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">
//!     <url>
//!         <loc>https://www.toddgriffin.me/</loc>
//!         <xhtml:link rel="alternate" hreflang="de" href="https://www.toddgriffin.me/de" />
//!         <lastmod>1998-01-15T04:20:00+00:00</lastmod>
//!         <changefreq>monthly</changefreq>
//!         <priority>0.69</priority>
//!     </url>
//! </urlset>
//! ```
//!
//! For more examples, check out the `examples` directory within the repository.
//!
//! ## Dates
//!
//! Dates are [`jiff::Zoned`] values, which are written in W3C Datetime format with seconds
//! precision (e.g. `1998-01-15T04:20:00+00:00`). `jiff` is re-exported as [`sitemap_rs::jiff`](jiff)
//! so you can always use a version that matches this crate.

pub use jiff;

pub mod image;
pub mod news;
pub mod sitemap;
pub mod sitemap_index;
pub mod sitemap_index_error;
pub mod url;
pub mod url_builder;
pub mod url_error;
pub mod url_set;
pub mod url_set_error;
pub mod video;
pub mod video_builder;
pub mod video_error;
mod xml;

pub const NAMESPACE: &str = "http://www.sitemaps.org/schemas/sitemap/0.9";
pub const XHTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
pub const IMAGE_NAMESPACE: &str = "http://www.google.com/schemas/sitemap-image/1.1";
pub const VIDEO_NAMESPACE: &str = "http://www.google.com/schemas/sitemap-video/1.1";
pub const NEWS_NAMESPACE: &str = "http://www.google.com/schemas/sitemap-news/0.9";
pub const ENCODING: &str = "UTF-8";
