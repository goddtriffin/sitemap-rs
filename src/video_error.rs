use std::error;
use std::fmt::{Display, Formatter};

/// An error when instantiating or generating sitemap videos.
#[derive(Debug)]
pub enum VideoError {
    /// Returned when a sitemap video's `description` is longer than `2048` characters.
    DescriptionTooLong(usize),

    /// Returned when a sitemap video's `duration` is below `1` second.
    DurationTooShort(u16),

    /// Returned when a sitemap video's `duration` is above `28,800` seconds (`8` hours).
    DurationTooLong(u16),

    /// Returned when a sitemap video's `rating` is below `0.0`.
    RatingTooLow(f32),

    /// Returned when a sitemap video's `rating` is above `5.0`.
    RatingTooHigh(f32),

    /// Returned when a sitemap video's `uploader` `name` is longer than `255` characters.
    UploaderNameTooLong(usize),

    /// Returned when a sitemap's video element has more than `32` tags.
    TooManyTags(usize),
}

impl error::Error for VideoError {}

impl Display for VideoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DescriptionTooLong(count) => {
                write!(
                    f,
                    "description cannot be longer than 2048 characters: {}",
                    count
                )
            }
            Self::DurationTooShort(duration) => {
                write!(f, "duration is below 1 (second): {}", duration)
            }
            Self::DurationTooLong(duration) => {
                write!(f, "duration is above 28,800 (seconds): {}", duration)
            }
            Self::RatingTooLow(rating) => {
                write!(f, "rating is below 0.0: {}", rating)
            }
            Self::RatingTooHigh(rating) => {
                write!(f, "rating is above 5.0: {}", rating)
            }
            Self::UploaderNameTooLong(count) => {
                write!(f, "uploader name is longer than 255 characters: {}", count)
            }
            Self::TooManyTags(count) => {
                write!(f, "must not have more than 32 tags: {}", count)
            }
        }
    }
}
