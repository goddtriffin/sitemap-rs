use crate::xml::write_text_element;
use crate::{RFC_3339_SECONDS_FORMAT, RFC_3339_USE_Z};
use chrono::{DateTime, FixedOffset};
use quick_xml::Writer;
use std::io::{self, Write};

/// A sitemap news.
#[derive(Debug, Clone)]
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

    pub(crate) fn write_xml<W: Write>(&self, writer: &mut Writer<W>) -> io::Result<()> {
        writer
            .create_element("news:news")
            .write_inner_content(|writer| {
                // add <news:publication>
                self.publication.write_xml(writer)?;

                // add <news:publication_date>
                write_text_element(
                    writer,
                    "news:publication_date",
                    &self
                        .publication_date
                        .to_rfc3339_opts(RFC_3339_SECONDS_FORMAT, RFC_3339_USE_Z),
                )?;

                // add <news:title>
                write_text_element(writer, "news:title", &self.title)
            })?;
        Ok(())
    }
}

/// The publication where the article appears.
#[derive(Debug, Clone)]
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

    pub(crate) fn write_xml<W: Write>(&self, writer: &mut Writer<W>) -> io::Result<()> {
        writer
            .create_element("news:publication")
            .write_inner_content(|writer| {
                // add <news:name>
                write_text_element(writer, "news:name", &self.name)?;

                // add <news:language>
                write_text_element(writer, "news:language", &self.language)
            })?;
        Ok(())
    }
}
