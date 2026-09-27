use crate::video_builder::VideoBuilder;
use crate::video_error::VideoError;
use crate::xml::{format_date, write_text_element};
use jiff::Zoned;
use quick_xml::Writer;
use quick_xml::events::BytesText;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::io::{self, Write};

/// A sitemap video.
///
/// It's required to provide either a `<video:content_loc>` or `<video:player_loc>` tag.
/// We recommend that your provide the `<video:content_loc>` tag, if possible.
/// This is the most effective way for Google to fetch your video content files.
/// If `<video:content_loc>` isn't available, provide `<video:player_loc>` as an alternative.
#[derive(Debug, Clone)]
pub struct Video {
    /// A URL pointing to the video thumbnail image file.
    pub thumbnail_location: String,

    /// The title of the video.
    ///
    /// We recommend that this match the video title displayed on the web page.
    pub title: String,

    /// A description of the video.
    ///
    /// Maximum 2048 characters.
    /// It must match the description displayed on the web page (it doesn't need to be a word-for-word match).
    pub description: String,

    /// A URL pointing to the actual video media file.
    ///
    /// The file must be one of the supported formats.
    /// - HTML and Flash aren't supported formats.
    /// - Must not be the same as the `<loc>` URL.
    /// - This is the equivalent of VideoObject.contentUrl in structured data.
    /// - Best practice: If you want to restrict access to your content but still have it crawled, ensure that Googlebot can access your content by using a reverse DNS lookup.
    pub content_location: String,

    /// A URL pointing to a player for a specific video.
    ///
    /// Usually this is the information in the src element of an `<embed>` tag.
    /// - Must not be the same as the `<loc>` URL.
    /// - For `YouTube` videos, this value is used rather than `video:content_loc`. This is the equivalent of VideoObject.embedUrl in structured data.
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
    pub expiration_date: Option<Zoned>,

    /// The rating of the video.
    ///
    /// Supported values are float numbers in the range 0.0 (low) to 5.0 (high), inclusive.
    pub rating: Option<f32>,

    /// The number of times the video has been viewed.
    pub view_count: Option<usize>,

    /// The date the video was first published, in W3C format.
    pub publication_date: Option<Zoned>,

    /// Whether the video is available with `SafeSearch`.
    ///
    /// If you omit this tag, the video is available when `SafeSearch` is turned on.
    pub family_friendly: Option<bool>,

    /// Whether to show or hide your video in search results from specific countries.
    ///
    /// Specify a space-delimited list of country codes in ISO 3166 format.
    /// Only one `<video:restriction>` tag can be used for each video.
    /// If there is no `<video:restriction>` tag, Google assumes that the video can be shown in all locations.
    /// Note that this tag only affects search results; it doesn't prevent a user from finding or playing your video in a restricted location though other means.
    pub restriction: Option<Restriction>,

    /// Whether to show or hide your video in search results on specified platform types.
    ///
    /// This is a list of space-delimited platform types.
    /// Note that this only affects search results on the specified device types; it does not prevent a user from playing your video on a restricted platform.
    /// Only one `<video:platform>` tag can appear for each video.
    /// If there is no `<video:platform>` tag, Google assumes that the video can be played on all platforms.
    pub platform: Option<Platform>,

    /// Indicates whether a subscription is required to view the video.
    pub requires_subscription: Option<bool>,

    /// The video uploader's name.
    ///
    /// Only one `<video:uploader>` is allowed per video.
    /// The string value can be a maximum of 255 characters.
    pub uploader: Option<Uploader>,

    /// Indicates whether the video is a live stream.
    pub live: Option<bool>,

    /// An arbitrary string tag describing the video.
    ///
    /// Tags are generally very short descriptions of key concepts associated with a video or piece of content.
    /// A single video could have several tags, although it might belong to only one category.
    /// For example, a video about grilling food may belong in the "grilling" category, but could be tagged "steak", "meat", "summer", and "outdoor".
    /// Create a new `<video:tag>` element for each tag associated with a video.
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
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        thumbnail_location: String,
        title: String,
        description: String,
        content_location: String,
        player_location: String,
        duration: Option<u16>,
        expiration_date: Option<Zoned>,
        rating: Option<f32>,
        view_count: Option<usize>,
        publication_date: Option<Zoned>,
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

    pub(crate) fn write_xml<W: Write>(&self, writer: &mut Writer<W>) -> io::Result<()> {
        writer
            .create_element("video:video")
            .write_inner_content(|writer| {
                // add <video:thumbnail_loc>
                write_text_element(writer, "video:thumbnail_loc", &self.thumbnail_location)?;

                // add <video:title>
                write_text_element(writer, "video:title", &self.title)?;

                // add <video:description>
                write_text_element(writer, "video:description", &self.description)?;

                // add <video:content_loc>
                write_text_element(writer, "video:content_loc", &self.content_location)?;

                // add <video:player_loc>
                write_text_element(writer, "video:player_loc", &self.player_location)?;

                // add <video:duration>, if it exists
                if let Some(d) = self.duration {
                    write_text_element(writer, "video:duration", &d.to_string())?;
                }

                // add <video:expiration_date>, if it exists
                if let Some(exp_date) = &self.expiration_date {
                    write_text_element(writer, "video:expiration_date", &format_date(exp_date))?;
                }

                // add <video:rating>, if it exists
                if let Some(r) = self.rating {
                    write_text_element(writer, "video:rating", &r.to_string())?;
                }

                // add <video:view_count>, if it exists
                if let Some(vc) = self.view_count {
                    write_text_element(writer, "video:view_count", &vc.to_string())?;
                }

                // add <video:publication_date>, if it exists
                if let Some(pub_date) = &self.publication_date {
                    write_text_element(writer, "video:publication_date", &format_date(pub_date))?;
                }

                // add <video:family_friendly>, if it exists
                if let Some(ff) = self.family_friendly {
                    write_text_element(writer, "video:family_friendly", yes_no(ff))?;
                }

                // add <video:restriction>, if it exists
                if let Some(restriction) = &self.restriction {
                    restriction.write_xml(writer)?;
                }

                // add <video:platform>, if it exists
                if let Some(platform) = &self.platform {
                    platform.write_xml(writer)?;
                }

                // add <video:requires_subscription>, if it exists
                if let Some(requires_sub) = self.requires_subscription {
                    write_text_element(
                        writer,
                        "video:requires_subscription",
                        yes_no(requires_sub),
                    )?;
                }

                // add <video:uploader>, if it exists
                if let Some(uploader) = &self.uploader {
                    uploader.write_xml(writer)?;
                }

                // add <video:live>, if it exists
                if let Some(l) = self.live {
                    write_text_element(writer, "video:live", yes_no(l))?;
                }

                // add <video:tag>, if it exists
                if let Some(tags) = &self.tags {
                    for t in tags {
                        write_text_element(writer, "video:tag", t)?;
                    }
                }

                Ok(())
            })?;
        Ok(())
    }
}

/// The value of a yes/no video tag.
const fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

/// Whether to show or hide your video in search results from specific countries.
///
/// Note that this tag only affects search results; it doesn't prevent a user from finding or playing your video in a restricted location though other means.
#[derive(Debug, Clone)]
pub struct Restriction {
    /// Specify a space-delimited list of country codes in ISO 3166 format.
    pub country_codes: BTreeSet<String>,

    /// Whether the video is allowed or denied in search results in the specified countries.
    /// Supported values are allow or deny.
    /// If allow, listed countries are allowed, unlisted countries are denied; if deny, listed countries are denied, unlisted countries are allowed.
    pub relationship: Relationship,
}

impl Restriction {
    #[must_use]
    pub const fn new(country_codes: BTreeSet<String>, relationship: Relationship) -> Self {
        Self {
            country_codes,
            relationship,
        }
    }

    pub(crate) fn write_xml<W: Write>(&self, writer: &mut Writer<W>) -> io::Result<()> {
        // set text as space-delimited country codes in ISO 3166 format
        let country_codes: String = self
            .country_codes
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>()
            .join(" ");

        writer
            .create_element("video:restriction")
            .with_attribute(("relationship", self.relationship.as_str()))
            .write_text_content(BytesText::new(&country_codes))?;
        Ok(())
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
    pub platforms: BTreeSet<PlatformType>,

    /// Specifies whether the video is restricted or permitted for the specified platforms.
    /// Supported values are allow or deny.
    /// If the allow value is used, any omitted platforms will be denied; if the deny value is used, any omitted platforms will be allowed.
    pub relationship: Relationship,
}

impl Platform {
    #[must_use]
    pub const fn new(platforms: BTreeSet<PlatformType>, relationship: Relationship) -> Self {
        Self {
            platforms,
            relationship,
        }
    }

    pub(crate) fn write_xml<W: Write>(&self, writer: &mut Writer<W>) -> io::Result<()> {
        // set text as space-delimited platform types
        let platform_types: String = self
            .platforms
            .iter()
            .map(PlatformType::as_str)
            .collect::<Vec<&str>>()
            .join(" ");

        writer
            .create_element("video:platform")
            .with_attribute(("relationship", self.relationship.as_str()))
            .write_text_content(BytesText::new(&platform_types))?;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlatformType {
    /// Mobile browsers, such as those on cellular phones or tablets.
    Mobile,
    /// TV browsers, such as those available through `GoogleTV` devices and game consoles.
    Tv,
    /// Traditional computer browsers on desktops and laptops.
    Web,
}

impl PlatformType {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Mobile => "mobile",
            Self::Tv => "tv",
            Self::Web => "web",
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
/// Only one `<video:uploader>` is allowed per video.
#[derive(Debug, Clone)]
pub struct Uploader {
    /// The string value can be a maximum of 255 characters.
    pub name: String,

    /// Specifies the URL of a webpage with additional information about this uploader.
    /// This URL must be in the same domain as the `<loc>` tag.
    pub info: Option<String>,
}

impl Uploader {
    #[must_use]
    pub const fn new(name: String, info: Option<String>) -> Self {
        Self { name, info }
    }

    pub(crate) fn write_xml<W: Write>(&self, writer: &mut Writer<W>) -> io::Result<()> {
        let mut uploader = writer.create_element("video:uploader");

        // set info attribute, if it exists
        if let Some(info) = &self.info {
            uploader = uploader.with_attribute(("info", info.as_str()));
        }

        // set uploader name as text
        uploader.write_text_content(BytesText::new(&self.name))?;
        Ok(())
    }
}
