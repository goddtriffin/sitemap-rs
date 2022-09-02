use std::error::Error;
use std::fmt::{Display, Formatter};

/// An error when instantiating or generating sitemap index files.
#[derive(Debug)]
pub enum SitemapIndexError {
    /// Returned when a \<sitemapindex\> contains more than `50,000` \<sitemap\>.
    TooManySitemaps(usize),
}

impl Error for SitemapIndexError {}

impl Display for SitemapIndexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManySitemaps(count) => {
                write!(f, "must not contain more than 50,000 sitemaps: {}", count)
            }
        }
    }
}
