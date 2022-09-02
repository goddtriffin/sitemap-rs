use crate::{RFC_3339_SECONDS_FORMAT, RFC_3339_USE_Z};
use chrono::{DateTime, FixedOffset};
use xml_builder::{XMLElement, XMLError};

/// A sitemap news.
#[derive(Debug)]
pub struct News {
    /// The publication where the article appears.
    pub publication: Publication,

    /// The article publication date in W3C format.
    ///
    /// Specify the original date and time when the article was published on your site.
    /// Don't specify the time when you added the article to your sitemap.
    pub publication_date: DateTime<FixedOffset>,

    /// The title of the news article.
    ///
    /// Tip: Google may shorten the title of the news article for space reasons when displaying the article on Google News.
    /// Include the title of the article as it appears on your site.
    /// Don't include the author name, publication name, or publication date in the News sitemap <title> tag.
    pub title: String,
}

impl News {
    #[must_use]
    pub const fn new(
        publication: Publication,
        publication_date: DateTime<FixedOffset>,
        title: String,
    ) -> Self {
        Self {
            publication,
            publication_date,
            title,
        }
    }

    /// # Errors
    ///
    /// Will return `XMLError` if there is a problem creating XML elements.
    pub fn to_xml(self) -> Result<XMLElement, XMLError> {
        let mut news: XMLElement = XMLElement::new("news:news");

        // add <news:publication>
        news.add_child(self.publication.to_xml()?)?;

        // add <news:publication_date>
        let mut publication_date: XMLElement = XMLElement::new("news:publication_date");
        publication_date.add_text(
            self.publication_date
                .to_rfc3339_opts(RFC_3339_SECONDS_FORMAT, RFC_3339_USE_Z),
        )?;
        news.add_child(publication_date)?;

        // add <news:title>
        let mut title: XMLElement = XMLElement::new("news:title");
        title.add_text(self.title)?;
        news.add_child(title)?;

        Ok(news)
    }
}

/// The publication where the article appears.
#[derive(Debug)]
pub struct Publication {
    /// The <name> tag is the name of the news publication.
    ///
    /// It must exactly match the name as it appears on your articles on news.google.com, except for anything in parentheses.
    pub name: String,

    /// The <language> tag is the language of your publication.
    ///
    /// Use an ISO 639 language code (two or three letters).
    /// Exception: For Simplified Chinese, use zh-cn and for Traditional Chinese, use zh-tw.
    pub language: String,
}

impl Publication {
    #[must_use]
    pub const fn new(name: String, language: String) -> Self {
        Self { name, language }
    }

    /// # Errors
    ///
    /// Will return `XMLError` if there is a problem creating XML elements.
    pub fn to_xml(self) -> Result<XMLElement, XMLError> {
        let mut publication: XMLElement = XMLElement::new("news:publication");

        // add <news:name>
        let mut name: XMLElement = XMLElement::new("news:name");
        name.add_text(self.name)?;
        publication.add_child(name)?;

        // add <news:language>
        let mut language: XMLElement = XMLElement::new("news:language");
        language.add_text(self.language)?;
        publication.add_child(language)?;

        Ok(publication)
    }
}
