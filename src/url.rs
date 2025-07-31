use crate::image::Image;
use crate::news::News;
use crate::url_builder::UrlBuilder;
use crate::url_error::UrlError;
use crate::video::Video;
use crate::{RFC_3339_SECONDS_FORMAT, RFC_3339_USE_Z};
use chrono::{DateTime, FixedOffset};
use std::fmt::{Display, Formatter};
use xml_builder::{XMLElement, XMLError};

/// The default priority of a sitemap.xml <url>.
pub const DEFAULT_PRIORITY: f32 = 0.5;

/// A \<url\> entry within a sitemap.xml.
#[derive(Debug, Clone)]
pub struct Url {
    /// URL of the page.
    ///
    /// This URL must begin with the protocol (such as http)
    /// This value must be less than 2,048 characters.
    pub location: String,

    /// URLs of alternative hreflang links
    pub links: Vec<UrlLink>,

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

impl Url {
    /// # Errors
    ///
    /// Will return `UrlError::PriorityTooLow` if `priority` is below `0.0`.
    /// Will return `UrlError::PriorityTooHigh` if `priority` is above `1.0`.
    /// Will return `UrlError::TooManyImages` if the length of `images` is above `1,000`.
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        location: String,
        links: Vec<UrlLink>,
        last_modified: Option<DateTime<FixedOffset>>,
        change_frequency: Option<ChangeFrequency>,
        priority: Option<f32>,
        images: Option<Vec<Image>>,
        videos: Option<Vec<Video>>,
        news: Option<News>,
    ) -> Result<Self, UrlError> {
        // make sure priority is within bounds: 0.0 <= priority <= 1.0
        if let Some(p) = priority {
            if p < 0.0 {
                return Err(UrlError::PriorityTooLow(p));
            }
            if p > 1.0 {
                return Err(UrlError::PriorityTooHigh(p));
            }
        }

        // make sure there aren't too many images
        let images: Option<Vec<Image>> = match images {
            None => None,
            Some(images) => {
                if images.len() > 1000 {
                    return Err(UrlError::TooManyImages(images.len()));
                }

                if images.is_empty() {
                    None
                } else {
                    Some(images)
                }
            }
        };

        Ok(Self {
            location,
            links,
            last_modified,
            change_frequency,
            priority,
            images,
            videos,
            news,
        })
    }

    #[must_use]
    pub const fn builder(location: String) -> UrlBuilder {
        UrlBuilder::new(location)
    }

    /// # Errors
    ///
    /// Will return `XMLError` if there is a problem creating XML elements.
    pub fn to_xml(self) -> Result<XMLElement, XMLError> {
        let mut url: XMLElement = XMLElement::new("url");

        // add <loc>
        let mut loc: XMLElement = XMLElement::new("loc");
        loc.add_text(self.location)?;
        url.add_child(loc)?;

        // add <xhtml:link>, if any exists
        for link in self.links {
            let mut xhtml_link = XMLElement::new("xhtml:link");
            xhtml_link.add_attribute("rel", "alternate");
            xhtml_link.add_attribute("hreflang", &link.hreflang);
            xhtml_link.add_attribute("href", &link.href);
            url.add_child(xhtml_link)?;
        }

        // add <lastmod>, if it exists
        if let Some(last_modified) = self.last_modified {
            let mut lastmod: XMLElement = XMLElement::new("lastmod");
            lastmod
                .add_text(last_modified.to_rfc3339_opts(RFC_3339_SECONDS_FORMAT, RFC_3339_USE_Z))?;
            url.add_child(lastmod)?;
        }

        // add <changefreq>, if it exists
        if let Some(change_frequency) = self.change_frequency {
            let mut changefreq: XMLElement = XMLElement::new("changefreq");
            changefreq.add_text(change_frequency.to_string())?;
            url.add_child(changefreq)?;
        }

        // add <priority>, if it exists
        if let Some(p) = self.priority {
            let mut priority: XMLElement = XMLElement::new("priority");
            priority.add_text(p.to_string())?;
            url.add_child(priority)?;
        }

        // add <image:image>, if any exist
        if let Some(images) = self.images {
            for image in images {
                url.add_child(image.to_xml()?)?;
            }
        }

        // add <video:video>, if any exist
        if let Some(videos) = self.videos {
            for video in videos {
                url.add_child(video.to_xml()?)?;
            }
        }

        // add <news:news>, if any exist
        if let Some(news) = self.news {
            url.add_child(news.to_xml()?)?;
        }

        Ok(url)
    }
}

#[derive(Debug, Clone)]
pub struct UrlLink {
    // Locale of the link
    // TODO: Use a predefined list of all possible locales?
    // If not, then consider using small string optimization
    pub hreflang: String,
    /// URL of alternative hreflang link
    ///
    /// This URL must begin with the protocol (such as http)
    /// This value must be less than 2,048 characters.
    pub href: String,
}

impl UrlLink {
    #[must_use]
    pub fn new(hreflang: String, href: String) -> Self {
        Self { hreflang, href }
    }
}

/// How frequently the page is likely to change.
///
/// This value provides general information to search engines and may not correlate exactly to how often they crawl the page.
///
/// The value "always" should be used to describe documents that change each time they are accessed.
/// The value "never" should be used to describe archived URLs.
///
/// Please note that the value of this tag is considered a hint and not a command.
/// Even though search engine crawlers may consider this information when making decisions, they may crawl pages marked "hourly" less frequently than that, and they may crawl pages marked "yearly" more frequently than that.
/// Crawlers may periodically crawl pages marked "never" so that they can handle unexpected changes to those pages.
#[derive(Debug, Copy, Clone)]
pub enum ChangeFrequency {
    /// The value "always" should be used to describe documents that change each time they are accessed.
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    /// The value "never" should be used to describe archived URLs.
    Never,
}

impl ChangeFrequency {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Always => "always",
            Self::Hourly => "hourly",
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
            Self::Yearly => "yearly",
            Self::Never => "never",
        }
    }
}

impl Display for ChangeFrequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
