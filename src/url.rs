use crate::image::Image;
use crate::news::News;
use crate::url_builder::UrlBuilder;
use crate::url_error::UrlError;
use crate::video::Video;
use crate::{RFC_3339_SECONDS_FORMAT, RFC_3339_USE_Z};
use chrono::{DateTime, FixedOffset};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use xml_builder::{XMLElement, XMLError};

/// The default priority of a sitemap.xml <url>.
pub const DEFAULT_PRIORITY: f32 = 0.5;

/// A \<url\> entry within a sitemap.xml.
#[derive(Debug, Clone)]
pub struct Url {
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

    /// Language Alternates for this URL.
    ///
    /// Alternates must not contain duplicate hreflang values.
    pub alternates: Option<Vec<Alternate>>,
}

impl Url {
    /// # Errors
    ///
    /// Will return `UrlError::PriorityTooLow` if `priority` is below `0.0`.
    /// Will return `UrlError::PriorityTooHigh` if `priority` is above `1.0`.
    /// Will return `UrlError::TooManyImages` if the length of `images` is above `1,000`.
    /// Will return `UrlError::DuplicateAlternateHreflangs` if `alternates` contain duplicate `hreflang` values.
    #[allow(clippy::complexity)]
    pub fn new(
        location: String,
        last_modified: Option<DateTime<FixedOffset>>,
        change_frequency: Option<ChangeFrequency>,
        priority: Option<f32>,
        images: Option<Vec<Image>>,
        videos: Option<Vec<Video>>,
        news: Option<News>,
        alternates: Option<Vec<Alternate>>,
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

        let alternates: Option<Vec<Alternate>> = match alternates {
            None => None,
            Some(alternates) => {
                let mut unique_hreflangs = HashSet::new();
                for alternate in &alternates {
                    if !unique_hreflangs.insert(&alternate.hreflang) {
                        return Err(UrlError::DuplicateAlternateHreflangs(
                            alternate.hreflang.clone(),
                            alternate.href.clone(),
                        ));
                    }
                }
                Some(alternates)
            }
        };

        Ok(Self {
            location,
            last_modified,
            change_frequency,
            priority,
            images,
            videos,
            news,
            alternates,
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
        loc.add_text(self.location.clone())?;
        url.add_child(loc)?;

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

        // add <xhtml:link>, if any exist
        if let Some(alternates) = self.alternates {
            for alternate in alternates {
                let mut alternate_link: XMLElement = XMLElement::new("xhtml:link");
                alternate_link.add_attribute("rel", "alternate");
                alternate_link.add_attribute("hreflang", &alternate.hreflang);
                alternate_link.add_attribute("href", &alternate.href);
                url.add_child(alternate_link)?;
            }
        }

        Ok(url)
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

/// Language Alternates for URL.
///
/// Alternates can be used to inform search engines about all language and region variants of a URL.
///
/// Possible values for hreflang are language codes (e.g. "en"), locales (e.g. "en-US") or "x-default".
#[derive(Debug, Clone)]
pub struct Alternate {
    pub hreflang: String,
    pub href: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_to_xml() {
        let url: Url = Url::builder(String::from("https://www.example.com/"))
            .last_modified(DateTime::parse_from_rfc3339("2023-01-01T13:37:00+00:00").unwrap())
            .change_frequency(ChangeFrequency::Weekly)
            .priority(DEFAULT_PRIORITY)
            .alternates(vec![Alternate {
                hreflang: String::from("en-US"),
                href: String::from("https://www.example.com/"),
            }])
            .push_alternate(
                String::from("x-default"),
                String::from("https://www.example.com/country-selector"),
            )
            .build()
            .expect("failed a <url> validation");

        let xml: XMLElement = url.to_xml().unwrap();
        let mut buf = Vec::<u8>::new();
        xml.render(&mut buf, true, true).unwrap();
        let result = String::from_utf8(buf).unwrap();
        assert_eq!(
            "\
            <url>\n\
                \t<loc>https://www.example.com/</loc>\n\
                \t<lastmod>2023-01-01T13:37:00+00:00</lastmod>\n\
                \t<changefreq>weekly</changefreq>\n\
                \t<priority>0.5</priority>\n\
                \t<xhtml:link href=\"https://www.example.com/\" hreflang=\"en-US\" rel=\"alternate\" />\n\
                \t<xhtml:link href=\"https://www.example.com/country-selector\" hreflang=\"x-default\" rel=\"alternate\" />\n\
            </url>\n",
            result
        );
    }
}
