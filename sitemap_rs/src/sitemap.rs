use crate::xml::write_text_element;
use crate::{RFC_3339_SECONDS_FORMAT, RFC_3339_USE_Z};
use chrono::{DateTime, FixedOffset};
use quick_xml::Writer;
use std::io::{self, Write};

/// Encapsulates information about an individual Sitemap.
#[derive(Debug, Clone)]
pub struct Sitemap {
    /// Identifies the location of the Sitemap.
    ///
    /// This location can be a Sitemap, an Atom file, RSS file or a simple text file.
    pub location: String,

    /// Identifies the time that the corresponding Sitemap file was modified.
    ///
    /// It does not correspond to the time that any of the pages listed in that Sitemap were changed.
    /// The value for the lastmod tag should be in W3C Datetime format.
    /// By providing the last modification timestamp, you enable search engine crawlers to retrieve only a subset of the Sitemaps in the index i.e. a crawler may only retrieve Sitemaps that were modified since a certain date.
    /// This incremental Sitemap fetching mechanism allows for the rapid discovery of new URLs on very large sites.
    pub last_modified: Option<DateTime<FixedOffset>>,
}

impl Sitemap {
    #[must_use]
    pub const fn new(location: String, last_modified: Option<DateTime<FixedOffset>>) -> Self {
        Self {
            location,
            last_modified,
        }
    }

    pub(crate) fn write_xml<W: Write>(&self, writer: &mut Writer<W>) -> io::Result<()> {
        writer
            .create_element("sitemap")
            .write_inner_content(|writer| {
                // add <loc>
                write_text_element(writer, "loc", &self.location)?;

                // add <lastmod>, if it exists
                if let Some(last_modified) = self.last_modified {
                    write_text_element(
                        writer,
                        "lastmod",
                        &last_modified.to_rfc3339_opts(RFC_3339_SECONDS_FORMAT, RFC_3339_USE_Z),
                    )?;
                }

                Ok(())
            })?;
        Ok(())
    }
}
