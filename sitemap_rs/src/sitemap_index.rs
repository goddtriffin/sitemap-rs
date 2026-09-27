use crate::NAMESPACE;
use crate::sitemap::Sitemap;
use crate::sitemap_index_error::SitemapIndexError;
use crate::xml::write_document;
use std::io::{self, Write};

/// Encapsulates information about all the Sitemaps in the file.
pub struct SitemapIndex {
    /// The namespace for the `<sitemapindex>`.
    pub xmlns: String,

    /// All the sitemaps that will become indexed.
    pub sitemaps: Vec<Sitemap>,
}

impl SitemapIndex {
    /// # Errors
    ///
    /// Will return `SitemapIndexError::TooManySitemaps` if the length of `sitemaps` is above `50,000`.
    pub fn new(sitemaps: Vec<Sitemap>) -> Result<Self, SitemapIndexError> {
        // SitemapIndex cannot contain more than 50,000 sitemaps
        if sitemaps.len() > 50_000 {
            return Err(SitemapIndexError::TooManySitemaps(sitemaps.len()));
        }

        Ok(Self {
            xmlns: NAMESPACE.to_string(),
            sitemaps,
        })
    }

    /// Writes this `SitemapIndex` as an XML document into `writer`.
    ///
    /// # Errors
    ///
    /// Will return `io::Error` if there is an IO Error dealing with the underlying writer.
    pub fn write<W: Write>(&self, writer: W) -> io::Result<()> {
        write_document(writer, |writer| {
            // create <sitemapindex>
            let sitemap_index = writer
                .create_element("sitemapindex")
                .with_attribute(("xmlns", self.xmlns.as_str()));

            if self.sitemaps.is_empty() {
                sitemap_index.write_empty()?;
                return Ok(());
            }

            // add each <sitemap>
            sitemap_index.write_inner_content(|writer| {
                for sitemap in &self.sitemaps {
                    sitemap.write_xml(writer)?;
                }
                Ok(())
            })?;
            Ok(())
        })
    }
}
