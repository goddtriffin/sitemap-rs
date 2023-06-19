use crate::video_builder::VideoBuilder;
use crate::video_error::VideoError;
use crate::{RFC_3339_SECONDS_FORMAT, RFC_3339_USE_Z};
use chrono::{DateTime, FixedOffset};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use xml_builder::{XMLElement, XMLError};

/// A sitemap video.
///
/// It's required to provide either a <video:content_loc> or <video:player_loc> tag.
/// We recommend that your provide the <video:content_loc> tag, if possible.
/// This is the most effective way for Google to fetch your video content files.
/// If <video:content_loc> isn't available, provide <video:player_loc> as an alternative.
#[derive(Debug, Clone)]
pub struct Video {
    /// A URL pointing to the video thumbnail image file.
    pub thumbnail_location: String,

    /// The title of the video.
    ///
    /// All HTML entities must be escaped or wrapped in a CDATA block.
    /// We recommend that this match the video title displayed on the web page.
    pub title: String,

    /// A description of the video.
    ///
    /// Maximum 2048 characters.
    /// All HTML entities must be escaped or wrapped in a CDATA block.
    /// It must match the description displayed on the web page (it doesn't need to be a word-for-word match).
    pub description: String,

    /// A URL pointing to the actual video media file.
    ///
    /// The file must be one of the supported formats.
    /// - HTML and Flash aren't supported formats.
    /// - Must not be the same as the <loc> URL.
    /// - This is the equivalent of VideoObject.contentUrl in structured data.
    /// - Best practice: If you want to restrict access to your content but still have it crawled, ensure that Googlebot can access your content by using a reverse DNS lookup.
    pub content_location: String,

    /// A URL pointing to a player for a specific video.
    ///
    /// Usually this is the information in the src element of an <embed> tag.
    /// - Must not be the same as the <loc> URL.
    /// - For YouTube videos, this value is used rather than video:content_loc. This is the equivalent of VideoObject.embedUrl in structured data.
    /// - Best practice: If you want to restrict access to your content but still have it crawled, ensure that Googlebot can access your content by using a reverse DNS lookup.
    pub player_location: String,

    /// The duration of the video, in seconds.
    ///
    /// Value must be from 1 to 28800 (8 hours) inclusive.
    pub duration: Option<u16>,

    /// The date after which the video is no longer be available, in W3C format.
    ///
    /// Omit this tag if your video does not expire.
    /// If present, Google Search won't show your video after this date.
    /// For recurring videos at the same URL, update the expiration date to the new expiration date.
    pub expiration_date: Option<DateTime<FixedOffset>>,

    /// The rating of the video.
    ///
    /// Supported values are float numbers in the range 0.0 (low) to 5.0 (high), inclusive.
    pub rating: Option<f32>,

    /// The number of times the video has been viewed.
    pub view_count: Option<usize>,

    /// The date the video was first published, in W3C format.
    pub publication_date: Option<DateTime<FixedOffset>>,

    /// Whether the video is available with SafeSearch.
    ///
    /// If you omit this tag, the video is available when SafeSearch is turned on.
    pub family_friendly: Option<bool>,

    /// Whether to show or hide your video in search results from specific countries.
    ///
    /// Specify a space-delimited list of country codes in ISO 3166 format.
    /// Only one <video:restriction> tag can be used for each video.
    /// If there is no <video:restriction> tag, Google assumes that the video can be shown in all locations.
    /// Note that this tag only affects search results; it doesn't prevent a user from finding or playing your video in a restricted location though other means.
    pub restriction: Option<Restriction>,

    /// Whether to show or hide your video in search results on specified platform types.
    ///
    /// This is a list of space-delimited platform types.
    /// Note that this only affects search results on the specified device types; it does not prevent a user from playing your video on a restricted platform.
    /// Only one <video:platform> tag can appear for each video.
    /// If there is no <video:platform> tag, Google assumes that the video can be played on all platforms.
    pub platform: Option<Platform>,

    /// Indicates whether a subscription is required to view the video.
    pub requires_subscription: Option<bool>,

    /// The video uploader's name.
    ///
    /// Only one <video:uploader> is allowed per video.
    /// The string value can be a maximum of 255 characters.
    pub uploader: Option<Uploader>,

    /// Indicates whether the video is a live stream.
    pub live: Option<bool>,

    /// An arbitrary string tag describing the video.
    ///
    /// Tags are generally very short descriptions of key concepts associated with a video or piece of content.
    /// A single video could have several tags, although it might belong to only one category.
    /// For example, a video about grilling food may belong in the "grilling" category, but could be tagged "steak", "meat", "summer", and "outdoor".
    /// Create a new <video:tag> element for each tag associated with a video.
    /// A maximum of 32 tags is permitted.
    pub tags: Option<Vec<String>>,
}

impl Video {
    /// # Errors
    ///
    /// Will return `VideoError::DescriptionTooLong` if `description` is longer than `2048` characters .
    /// Will return `VideoError::DurationTooShort` if `duration` is below `1` second.
    /// Will return `VideoError::DurationTooLong` if `duration` is above `28,800` seconds (`8` hours).
    /// Will return `VideoError::RatingTooLow` if `rating` is below `0.0`.
    /// Will return `VideoError::RatingTooHigh` if `rating` is above `5.0`.
    /// Will return `VideoError::UploaderNameTooLong` if `uploader` `name` is longer than `255` characters.
    /// Will return `VideoError::TooManyTags` if there are more than `32` `tags`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        thumbnail_location: String,
        title: String,
        description: String,
        content_location: String,
        player_location: String,
        duration: Option<u16>,
        expiration_date: Option<DateTime<FixedOffset>>,
        rating: Option<f32>,
        view_count: Option<usize>,
        publication_date: Option<DateTime<FixedOffset>>,
        family_friendly: Option<bool>,
        restriction: Option<Restriction>,
        platform: Option<Platform>,
        requires_subscription: Option<bool>,
        uploader: Option<Uploader>,
        live: Option<bool>,
        tags: Option<Vec<String>>,
    ) -> Result<Self, VideoError> {
        // description must be no longer than `2048` characters
        if description.len() > 2048 {
            return Err(VideoError::DescriptionTooLong(description.len()));
        }

        if let Some(duration) = duration {
            // duration should be at least `1` second
            if duration < 1 {
                return Err(VideoError::DurationTooShort(duration));
            }
            // duration should be no longer than `28,800` seconds (8 hours)
            if duration > 28800 {
                return Err(VideoError::DurationTooLong(duration));
            }
        }

        if let Some(rating) = rating {
            // rating should be no lower than `0.0`
            if rating < 0.0 {
                return Err(VideoError::RatingTooLow(rating));
            }

            // rating should be no higher than `5.0`
            if rating > 5.0 {
                return Err(VideoError::RatingTooHigh(rating));
            }
        }

        if let Some(uploader) = &uploader {
            // uploader name should be no longer than `255` characters
            if uploader.name.len() > 255 {
                return Err(VideoError::UploaderNameTooLong(uploader.name.len()));
            }
        }

        if let Some(tags) = &tags {
            // there should not be more than `32` tags
            if tags.len() > 32 {
                return Err(VideoError::TooManyTags(tags.len()));
            }
        }

        Ok(Self {
            thumbnail_location,
            title,
            description,
            content_location,
            player_location,
            duration,
            expiration_date,
            rating,
            view_count,
            publication_date,
            family_friendly,
            restriction,
            platform,
            requires_subscription,
            uploader,
            live,
            tags,
        })
    }

    #[must_use]
    pub const fn builder(
        thumbnail_location: String,
        title: String,
        description: String,
        content_location: String,
        player_location: String,
    ) -> VideoBuilder {
        VideoBuilder::new(
            thumbnail_location,
            title,
            description,
            content_location,
            player_location,
        )
    }

    /// # Errors
    ///
    /// Will return `XMLError` if there is a problem creating XML elements.
    #[allow(clippy::too_many_lines)]
    pub fn to_xml(self) -> Result<XMLElement, XMLError> {
        let mut video: XMLElement = XMLElement::new("video:video");

        // add <video:thumbnail_loc>
        let mut thumbnail_loc: XMLElement = XMLElement::new("video:thumbnail_loc");
        thumbnail_loc.add_text(self.thumbnail_location)?;
        video.add_child(thumbnail_loc)?;

        // add <video:title>
        let mut title: XMLElement = XMLElement::new("video:title");
        title.add_text(self.title)?;
        video.add_child(title)?;

        // add <video:description>
        let mut description: XMLElement = XMLElement::new("video:description");
        description.add_text(self.description)?;
        video.add_child(description)?;

        // add <video:content_loc>
        let mut content_loc: XMLElement = XMLElement::new("video:content_loc");
        content_loc.add_text(self.content_location)?;
        video.add_child(content_loc)?;

        // add <video:player_loc>
        let mut player_loc: XMLElement = XMLElement::new("video:player_loc");
        player_loc.add_text(self.player_location)?;
        video.add_child(player_loc)?;

        // add <video:duration>, if it exists
        if let Some(d) = self.duration {
            let mut duration: XMLElement = XMLElement::new("video:duration");
            duration.add_text(d.to_string())?;
            video.add_child(duration)?;
        }

        // add <video:expiration_date>, if it exists
        if let Some(exp_date) = self.expiration_date {
            let mut expiration_date: XMLElement = XMLElement::new("video:expiration_date");
            expiration_date
                .add_text(exp_date.to_rfc3339_opts(RFC_3339_SECONDS_FORMAT, RFC_3339_USE_Z))?;
            video.add_child(expiration_date)?;
        }

        // add <video:rating>, if it exists
        if let Some(r) = self.rating {
            let mut rating: XMLElement = XMLElement::new("video:rating");
            rating.add_text(r.to_string())?;
            video.add_child(rating)?;
        }

        // add <video:view_count>, if it exists
        if let Some(vc) = self.view_count {
            let mut view_count: XMLElement = XMLElement::new("video:view_count");
            view_count.add_text(vc.to_string())?;
            video.add_child(view_count)?;
        }

        // add <video:publication_date>, if it exists
        if let Some(pub_date) = self.publication_date {
            let mut publication_date: XMLElement = XMLElement::new("video:publication_date");
            publication_date
                .add_text(pub_date.to_rfc3339_opts(RFC_3339_SECONDS_FORMAT, RFC_3339_USE_Z))?;
            video.add_child(publication_date)?;
        }

        // add <video:family_friendly>, if it exists
        if let Some(ff) = self.family_friendly {
            let ff: &str = if ff { "yes" } else { "no" };
            let mut family_friendly: XMLElement = XMLElement::new("video:family_friendly");
            family_friendly.add_text(ff.to_string())?;
            video.add_child(family_friendly)?;
        }

        // add <video:restriction>, if it exists
        if let Some(restriction) = self.restriction {
            video.add_child(restriction.to_xml()?)?;
        }

        // add <video:platform>, if it exists
        if let Some(platform) = self.platform {
            video.add_child(platform.to_xml()?)?;
        }

        // add <video:requires_subscription>, if it exists
        if let Some(requires_sub) = self.requires_subscription {
            let requires_sub: &str = if requires_sub { "yes" } else { "no" };
            let mut requires_subscription: XMLElement =
                XMLElement::new("video:requires_subscription");
            requires_subscription.add_text(requires_sub.to_string())?;
            video.add_child(requires_subscription)?;
        }

        // add <video:uploader>, if it exists
        if let Some(uploader) = self.uploader {
            video.add_child(uploader.to_xml()?)?;
        }

        // add <video:live>, if it exists
        if let Some(l) = self.live {
            let l: &str = if l { "yes" } else { "no" };
            let mut live: XMLElement = XMLElement::new("video:live");
            live.add_text(l.to_string())?;
            video.add_child(live)?;
        }

        // add <video:tag>, if it exists
        if let Some(tags) = self.tags {
            for t in tags {
                let mut tag: XMLElement = XMLElement::new("video:tag");
                tag.add_text(t)?;
                video.add_child(tag)?;
            }
        }

        Ok(video)
    }
}

/// Whether to show or hide your video in search results from specific countries.
///
/// Note that this tag only affects search results; it doesn't prevent a user from finding or playing your video in a restricted location though other means.
#[derive(Debug, Clone)]
pub struct Restriction {
    /// Specify a space-delimited list of country codes in ISO 3166 format.
    pub country_codes: HashSet<String>,

    /// Whether the video is allowed or denied in search results in the specified countries.
    /// Supported values are allow or deny.
    /// If allow, listed countries are allowed, unlisted countries are denied; if deny, listed countries are denied, unlisted countries are allowed.
    pub relationship: Relationship,
}

impl Restriction {
    #[must_use]
    pub const fn new(country_codes: HashSet<String>, relationship: Relationship) -> Self {
        Self {
            country_codes,
            relationship,
        }
    }

    /// # Errors
    ///
    /// Will return `XMLError` if there is a problem creating XML elements.
    pub fn to_xml(self) -> Result<XMLElement, XMLError> {
        let mut restriction: XMLElement = XMLElement::new("video:restriction");

        // set relationship attribute
        restriction.add_attribute("relationship", self.relationship.as_str());

        // set text as space-delimited country codes in ISO 3166 format
        let country_codes: String = self.country_codes.into_iter().collect::<Vec<_>>().join(" ");
        restriction.add_text(country_codes)?;

        Ok(restriction)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Relationship {
    Allow,
    Deny,
}

impl Relationship {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
        }
    }
}

impl Display for Relationship {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Whether to show or hide your video in search results on specified platform types.
///
/// Note that this only affects search results on the specified device types; it does not prevent a user from playing your video on a restricted platform.
#[derive(Debug, Clone)]
pub struct Platform {
    pub platforms: HashSet<PlatformType>,

    /// Specifies whether the video is restricted or permitted for the specified platforms.
    /// Supported values are allow or deny.
    /// If the allow value is used, any omitted platforms will be denied; if the deny value is used, any omitted platforms will be allowed.
    pub relationship: Relationship,
}

impl Platform {
    #[must_use]
    pub const fn new(platforms: HashSet<PlatformType>, relationship: Relationship) -> Self {
        Self {
            platforms,
            relationship,
        }
    }

    /// # Errors
    ///
    /// Will return `XMLError` if there is a problem creating XML elements.
    pub fn to_xml(self) -> Result<XMLElement, XMLError> {
        let mut platform: XMLElement = XMLElement::new("video:platform");

        // set relationship attribute
        platform.add_attribute("relationship", self.relationship.as_str());

        // set text as space-delimited platform types
        let platform_types: String = self
            .platforms
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ");
        platform.add_text(platform_types)?;

        Ok(platform)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlatformType {
    /// Traditional computer browsers on desktops and laptops.
    Web,
    /// Mobile browsers, such as those on cellular phones or tablets.
    Mobile,
    /// TV browsers, such as those available through GoogleTV devices and game consoles.
    Tv,
}

impl PlatformType {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Web => "web",
            Self::Mobile => "mobile",
            Self::Tv => "tv",
        }
    }
}

impl Display for PlatformType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// The video uploader's name.
///
/// Only one <video:uploader> is allowed per video.
#[derive(Debug, Clone)]
pub struct Uploader {
    /// The string value can be a maximum of 255 characters.
    pub name: String,

    /// Specifies the URL of a webpage with additional information about this uploader.
    /// This URL must be in the same domain as the <loc> tag.
    pub info: Option<String>,
}

impl Uploader {
    #[must_use]
    pub const fn new(name: String, info: Option<String>) -> Self {
        Self { name, info }
    }

    /// # Errors
    ///
    /// Will return `XMLError` if there is a problem creating XML elements.
    pub fn to_xml(self) -> Result<XMLElement, XMLError> {
        let mut uploader: XMLElement = XMLElement::new("video:uploader");

        // set info attribute, if it exists
        if let Some(info) = self.info {
            uploader.add_attribute("info", info.as_str());
        }

        // set uploader name as text
        uploader.add_text(self.name)?;

        Ok(uploader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_uploader_to_xml() {
        let uploader: Uploader = Uploader::new(
            String::from("UserName"),
            Some(String::from("https://www.example.com/users/UserName")),
        );

        let xml: XMLElement = uploader.to_xml().unwrap();
        let mut buf = Vec::<u8>::new();
        xml.render(&mut buf, true, true).unwrap();
        let result = String::from_utf8(buf).unwrap();
        assert_eq!(
            "\
            <video:uploader info=\"https://www.example.com/users/UserName\">\
                UserName\
            </video:uploader>\n",
            result
        );
    }

    #[test]
    fn test_platform_to_xml() {
        let platform: Platform = Platform::new(
            HashSet::from([PlatformType::Web, PlatformType::Tv]),
            Relationship::Allow,
        );

        let xml: XMLElement = platform.to_xml().unwrap();
        let mut buf = Vec::<u8>::new();
        xml.render(&mut buf, true, true).unwrap();
        let result = String::from_utf8(buf).unwrap();
        // XXX: Values in the final result are random and not sorted. Probably
        // because `Platform` is built from a HashSet and therefore has no order.
        // It is weird though that `XMLElement.render(&mut buf, true, true)` does not
        // sort the values as the second parameter is `should_sort`.
        assert!(result.contains("<video:platform relationship=\"allow\">"));
        assert!(result.contains("web tv") || result.contains("tv web"));
        assert!(result.contains("</video:platform>"));
    }

    #[test]
    fn test_restriction_to_xml() {
        let restriction: Restriction = Restriction::new(
            HashSet::from([String::from("IE"), String::from("GB")]),
            Relationship::Allow,
        );

        let xml: XMLElement = restriction.to_xml().unwrap();
        let mut buf = Vec::<u8>::new();
        xml.render(&mut buf, true, true).unwrap();
        let result = String::from_utf8(buf).unwrap();
        // XXX: Values in the final result are random and not sorted. Probably
        // because `Restriction` is built from a HashSet and therefore has no order.
        // It is weird though that `XMLElement.render(&mut buf, true, true)` does not
        // sort the values as the second parameter is `should_sort`.
        assert!(result.contains("<video:restriction relationship=\"allow\">"));
        assert!(result.contains("IE GB") || result.contains("GB IE"));
        assert!(result.contains("</video:restriction>"));
    }

    #[test]
    fn test_video_to_xml() {
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
        It is weird that `XMLElement.render(&mut buf, true, true)` does not sort them
        as the second parameter is `should_sort`.
        .restriction(Restriction::new(
            HashSet::from([String::from("IE"), String::from("GB")]),
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

        let xml: XMLElement = video.to_xml().unwrap();
        let mut buf = Vec::<u8>::new();
        xml.render(&mut buf, true, true).unwrap();
        let result = String::from_utf8(buf).unwrap();
        assert_eq!(
            "\
            <video:video>\n\
                \t<video:thumbnail_loc>https://www.example.com/thumbnail.jpg</video:thumbnail_loc>\n\
                \t<video:title>Video Title</video:title>\n\
                \t<video:description>Video description</video:description>\n\
                \t<video:content_loc>https://www.example.com/content_location.mp4</video:content_loc>\n\
                \t<video:player_loc>https://www.example.com/player_location.php</video:player_loc>\n\
                \t<video:duration>600</video:duration>\n\
                \t<video:expiration_date>2025-01-01T13:37:00+00:00</video:expiration_date>\n\
                \t<video:rating>4.2</video:rating>\n\
                \t<video:view_count>12345</video:view_count>\n\
                \t<video:publication_date>2009-01-01T13:37:00+00:00</video:publication_date>\n\
                \t<video:family_friendly>yes</video:family_friendly>\n\
                \t<video:requires_subscription>yes</video:requires_subscription>\n\
                \t<video:uploader info=\"https://www.example.com/users/UserName\">UserName</video:uploader>\n\
                \t<video:live>no</video:live>\n\
                \t<video:tag>video tag 1</video:tag>\n\
                \t<video:tag>video tag 2</video:tag>\n\
            </video:video>\n",
            result
        );
    }
}
