use crate::image::Image;
use crate::news::News;
use crate::url::{ChangeFrequency, Url};
use crate::url_error::UrlError;
use crate::video::Video;
use chrono::{DateTime, FixedOffset};

/// A \<url\> entry within a sitemap.xml.
///
/// This is a Builder for Url.
#[derive(Debug, Clone)]
pub struct UrlBuilder {
    /// URL of the page.
    ///
    /// This URL must begin with the protocol (such as http) and end with a trailing slash, if your web server requires it.
    /// This value must be less than 2,048 characters.
    pub location: String,

    /// The date of last modification of the page.
    ///
    /// This date should be in W3C Datetime format.
    /// This format allows you to omit the time portion, if desired, and use YYYY-MM-DD.
    /// Note that the date must be set to the date the linked page was last modified, not when the sitemap is generated.
    /// Note also that this tag is separate from the If-Modified-Since (304) header the server can return, and search engines may use the information from both sources differently.
    pub last_modified: Option<DateTime<FixedOffset>>,

    /// How frequently the page is likely to change.
    ///
    /// This value provides general information to search engines and may not correlate exactly to how often they crawl the page.
    /// The value "always" should be used to describe documents that change each time they are accessed.
    /// The value "never" should be used to describe archived URLs.
    /// Please note that the value of this tag is considered a hint and not a command.
    /// Even though search engine crawlers may consider this information when making decisions, they may crawl pages marked "hourly" less frequently than that, and they may crawl pages marked "yearly" more frequently than that.
    /// Crawlers may periodically crawl pages marked "never" so that they can handle unexpected changes to those pages.
    pub change_frequency: Option<ChangeFrequency>,

    /// The priority of this URL relative to other URLs on your site.
    ///
    /// Valid values range from 0.0 to 1.0.
    /// This value does not affect how your pages are compared to pages on other sites—it only lets the search engines know which pages you deem most important for the crawlers.
    /// The default priority of a page is 0.5.
    /// Please note that the priority you assign to a page is not likely to influence the position of your URLs in a search engine's result pages.
    /// Search engines may use this information when selecting between URLs on the same site, so you can use this tag to increase the likelihood that your most important pages are present in a search index.
    /// Also, please note that assigning a high priority to all the URLs on your site is not likely to help you.
    /// Since the priority is relative, it is only used to select between URLs on your site.
    pub priority: Option<f32>,

    /// Images associated with this URL.
    pub images: Option<Vec<Image>>,

    /// Videos associated with this URL.
    pub videos: Option<Vec<Video>>,

    /// News associated with this URL.
    pub news: Option<News>,
}

impl UrlBuilder {
    #[must_use]
    pub const fn new(location: String) -> Self {
        Self {
            location,
            last_modified: None,
            change_frequency: None,
            priority: None,
            images: None,
            videos: None,
            news: None,
        }
    }

    pub fn last_modified(&mut self, last_modified: DateTime<FixedOffset>) -> &mut Self {
        self.last_modified = Some(last_modified);
        self
    }

    pub fn change_frequency(&mut self, change_frequency: ChangeFrequency) -> &mut Self {
        self.change_frequency = Some(change_frequency);
        self
    }

    pub fn priority(&mut self, priority: f32) -> &mut Self {
        self.priority = Some(priority);
        self
    }

    pub fn images(&mut self, images: Vec<Image>) -> &mut Self {
        self.images = Some(images);
        self
    }

    pub fn videos(&mut self, videos: Vec<Video>) -> &mut Self {
        self.videos = Some(videos);
        self
    }

    pub fn news(&mut self, news: News) -> &mut Self {
        self.news = Some(news);
        self
    }

    /// # Errors
    ///
    /// Will return `UrlError::PriorityTooLow` if `priority` is below `0.0`.
    /// Will return `UrlError::PriorityTooHigh` if `priority` is above `1.0`.
    /// Will return `UrlError::TooManyImages` if the length of `images` is above `1,000`.
    pub fn build(&self) -> Result<Url, UrlError> {
        Url::new(
            self.location.clone(),
            self.last_modified,
            self.change_frequency,
            self.priority,
            self.images.clone(),
            self.videos.clone(),
            self.news.clone(),
        )
    }
}
