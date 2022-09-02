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
