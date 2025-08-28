use std::error;
use std::fmt::{Display, Formatter};

/// An error when instantiating or generating sitemap URLs.
#[derive(Debug)]
pub enum UrlError {
    /// Returned when a sitemap URL entry's `loc` is 2,048 characters or more.
    LocationTooLong(String),

    /// Returned when a sitemap URL entry's `priority` is below 0.
    PriorityTooLow(f32),

    /// Returned when a sitemap URL entry's `priority` is above 1.
    PriorityTooHigh(f32),

    /// Returned when a sitemap URL entry's `images` is more than 1,000.
    TooManyImages(usize),
}

impl error::Error for UrlError {}

impl Display for UrlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocationTooLong(location) => {
                write!(f, "location must be less than 2,048 characters: {location}")
            }
            Self::PriorityTooLow(priority) => {
                write!(f, "priority must not be below 0.0: {priority}")
            }
            Self::PriorityTooHigh(priority) => {
                write!(f, "priority must not be above 1.0: {priority}")
            }
            Self::TooManyImages(count) => {
                write!(f, "must not contain more tha 1,000 images: {count}")
            }
        }
    }
}
