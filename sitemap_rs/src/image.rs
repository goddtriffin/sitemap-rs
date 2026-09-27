use crate::xml::write_text_element;
use quick_xml::Writer;
use std::io::{self, Write};

/// A sitemap image.
#[derive(Debug, Clone)]
pub struct Image {
    /// The URL of the image.
    ///
    /// In some cases, the image URL may not be on the same domain as your main site.
    /// This is fine, as long as both domains are verified in Search Console.
    /// If, for example, you use a content delivery network such as Google Sites to host your images, make sure that the hosting site is verified in Search Console.
    /// In addition, make sure that your robots.txt file doesn't disallow the crawling of any content you want indexed.
    pub location: String,
}

impl Image {
    #[must_use]
    pub const fn new(location: String) -> Self {
        Self { location }
    }

    pub(crate) fn write_xml<W: Write>(&self, writer: &mut Writer<W>) -> io::Result<()> {
        writer
            .create_element("image:image")
            .write_inner_content(|writer| {
                // add <image:loc>
                write_text_element(writer, "image:loc", &self.location)
            })?;
        Ok(())
    }
}
