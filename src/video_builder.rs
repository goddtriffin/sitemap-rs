use crate::video::{Platform, Restriction, Uploader, Video};
use crate::video_error::VideoError;
use chrono::{DateTime, FixedOffset};

/// A sitemap video.
///
/// This is a Builder for Video.
///
/// It's required to provide either a <video:content_loc> or <video:player_loc> tag.
/// We recommend that your provide the <video:content_loc> tag, if possible.
/// This is the most effective way for Google to fetch your video content files.
/// If <video:content_loc> isn't available, provide <video:player_loc> as an alternative.
#[derive(Debug, Clone)]
pub struct VideoBuilder {
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

impl VideoBuilder {
    #[must_use]
    pub const fn new(
        thumbnail_location: String,
        title: String,
        description: String,
        content_location: String,
        player_location: String,
    ) -> Self {
        Self {
            thumbnail_location,
            title,
            description,
            content_location,
            player_location,
            duration: None,
            expiration_date: None,
            rating: None,
            view_count: None,
            publication_date: None,
            family_friendly: None,
            restriction: None,
            platform: None,
            requires_subscription: None,
            uploader: None,
            live: None,
            tags: None,
        }
    }

    pub fn duration(&mut self, duration: u16) -> &mut Self {
        self.duration = Some(duration);
        self
    }

    pub fn expiration_date(&mut self, expiration_date: DateTime<FixedOffset>) -> &mut Self {
        self.expiration_date = Some(expiration_date);
        self
    }

    pub fn rating(&mut self, rating: f32) -> &mut Self {
        self.rating = Some(rating);
        self
    }

    pub fn view_count(&mut self, view_count: usize) -> &mut Self {
        self.view_count = Some(view_count);
        self
    }

    pub fn publication_date(&mut self, publication_date: DateTime<FixedOffset>) -> &mut Self {
        self.publication_date = Some(publication_date);
        self
    }

    pub fn family_friendly(&mut self, family_friendly: bool) -> &mut Self {
        self.family_friendly = Some(family_friendly);
        self
    }

    pub fn restriction(&mut self, restriction: Restriction) -> &mut Self {
        self.restriction = Some(restriction);
        self
    }

    pub fn platform(&mut self, platform: Platform) -> &mut Self {
        self.platform = Some(platform);
        self
    }

    pub fn requires_subscription(&mut self, requires_subscription: bool) -> &mut Self {
        self.requires_subscription = Some(requires_subscription);
        self
    }

    pub fn uploader(&mut self, uploader: Uploader) -> &mut Self {
        self.uploader = Some(uploader);
        self
    }

    pub fn live(&mut self, live: bool) -> &mut Self {
        self.live = Some(live);
        self
    }

    pub fn tags(&mut self, tags: Vec<String>) -> &mut Self {
        self.tags = Some(tags);
        self
    }

    /// # Errors
    ///
    /// Will return `VideoError::DescriptionTooLong` if `description` is longer than `2048` characters .
    /// Will return `VideoError::DurationTooShort` if `duration` is below `1` second.
    /// Will return `VideoError::DurationTooLong` if `duration` is above `28,800` seconds (`8` hours).
    /// Will return `VideoError::RatingTooLow` if `rating` is below `0.0`.
    /// Will return `VideoError::RatingTooHigh` if `rating` is above `5.0`.
    /// Will return `VideoError::UploaderNameTooLong` if `uploader` `name` is longer than `255` characters.
    /// Will return `VideoError::TooManyTags` if there are more than `32` `tags`.
    pub fn build(&self) -> Result<Video, VideoError> {
        Video::new(
            self.thumbnail_location.clone(),
            self.title.clone(),
            self.description.clone(),
            self.content_location.clone(),
            self.player_location.clone(),
            self.duration,
            self.expiration_date,
            self.rating,
            self.view_count,
            self.publication_date,
            self.family_friendly,
            self.restriction.clone(),
            self.platform.clone(),
            self.requires_subscription,
            self.uploader.clone(),
            self.live,
            self.tags.clone(),
        )
    }
}
