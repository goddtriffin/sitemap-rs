use crate::sitemap::Sitemap;
use crate::sitemap_index_error::SitemapIndexError;
use crate::{ENCODING, NAMESPACE};
use std::io::Write;
use xml_builder::{XML, XMLBuilder, XMLElement, XMLError, XMLVersion};

/// Encapsulates information about all the Sitemaps in the file.
pub struct SitemapIndex {
    /// The XML version.
    pub xml_version: XMLVersion,

    /// The XML encoding.
    pub xml_encoding: String,

    /// The namespace for the \<sitemapindex\>.
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
            xml_version: XMLVersion::XML1_0,
            xml_encoding: ENCODING.to_string(),
            xmlns: NAMESPACE.to_string(),
            sitemaps,
        })
    }

    /// # Errors
    ///
    /// Will return `XMLError` if there is a problem creating XML elements.
    pub fn to_xml(self) -> Result<XML, XMLError> {
        // create XML document
        let mut xml = XMLBuilder::new()
            .version(self.xml_version)
            .encoding(self.xml_encoding)
            .build();

        // create <sitemapindex>
        let mut sitemap_index: XMLElement = XMLElement::new("sitemapindex");
        sitemap_index.add_attribute("xmlns", self.xmlns.as_str());

        // add each <sitemap>
        for sitemap in self.sitemaps {
            sitemap_index.add_child(sitemap.to_xml()?)?;
        }

        // set root element and we're done!
        xml.set_root_element(sitemap_index);
        Ok(xml)
    }

    /// # Errors
    ///
    /// Will return `XMLError` if there is an IO Error dealing with the
    /// underlying writer or if there is an error generating XML.
    pub fn write<W: Write>(self, writer: W) -> Result<(), XMLError> {
        let xml: XML = self.to_xml()?;
        xml.generate(writer)
    }
}
