use xml_builder::{XMLElement, XMLError};

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

    /// # Errors
    ///
    /// Will return `XMLError` if there is a problem creating XML elements.
    pub fn to_xml(self) -> Result<XMLElement, XMLError> {
        let mut image: XMLElement = XMLElement::new("image:image");

        // add <image:loc>
        let mut loc: XMLElement = XMLElement::new("image:loc");
        loc.add_text(self.location)?;
        image.add_child(loc)?;

        Ok(image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_to_xml() {
        let image: Image = Image::new(String::from("https://www.example.com/image.jpg"));

        let xml: XMLElement = image.to_xml().unwrap();
        let mut buf = Vec::<u8>::new();
        xml.render(&mut buf, true, true).unwrap();
        let result = String::from_utf8(buf).unwrap();
        assert_eq!(
            "\
            <image:image>\n\
                \t<image:loc>https://www.example.com/image.jpg</image:loc>\n\
            </image:image>\n",
            result
        );
    }
}
