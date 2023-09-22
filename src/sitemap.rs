use crate::{RFC_3339_SECONDS_FORMAT, RFC_3339_USE_Z};
use chrono::{DateTime, FixedOffset};
use xml_builder::{XMLElement, XMLError};

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

    /// # Errors
    ///
    /// Will return `XMLError` if there is a problem creating XML elements.
    pub fn to_xml(self) -> Result<XMLElement, XMLError> {
        let mut sitemap: XMLElement = XMLElement::new("sitemap");

        // add <loc>
        let mut loc: XMLElement = XMLElement::new("loc");
        loc.add_text(self.location)?;
        sitemap.add_child(loc)?;

        // add <lastmod>, if it exists
        if let Some(last_modified) = self.last_modified {
            let mut last_mod: XMLElement = XMLElement::new("lastmod");
            last_mod
                .add_text(last_modified.to_rfc3339_opts(RFC_3339_SECONDS_FORMAT, RFC_3339_USE_Z))?;
            sitemap.add_child(last_mod)?;
        }

        Ok(sitemap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_to_xml() {
        let sitemap: Sitemap = Sitemap::new(
            String::from("https://www.example.com/sitemap1.xml.gz"),
            Some(DateTime::parse_from_rfc3339("2023-01-01T13:37:00+00:00").unwrap()),
        );

        let xml: XMLElement = sitemap.to_xml().unwrap();
        let mut buf = Vec::<u8>::new();
        xml.render(&mut buf, true, true).unwrap();
        let result = String::from_utf8(buf).unwrap();
        assert_eq!(
            "\
            <sitemap>\n\
                \t<loc>https://www.example.com/sitemap1.xml.gz</loc>\n\
                \t<lastmod>2023-01-01T13:37:00+00:00</lastmod>\n\
            </sitemap>\n",
            result
        );
    }
}
