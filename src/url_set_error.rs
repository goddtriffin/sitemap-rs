use std::error::Error;
use std::fmt::{Display, Formatter};

/// An error when instantiating or generating sitemaps.
#[derive(Debug)]
pub enum UrlSetError {
    /// Returned when a \<urlset\> contains more than `50,000` \<url\>.
    TooManyUrls(usize),

    /// Returned when a \<urlset\> contains more than `1,000` \<news\>.
    TooMuchNews(usize),
}

impl Error for UrlSetError {}

impl Display for UrlSetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyUrls(count) => {
                write!(f, "must not contain more than 50,000 URLs: {count}")
            }
            Self::TooMuchNews(count) => {
                write!(f, "must not contains more than 1,000 news URLs: {count}")
            }
        }
    }
}
