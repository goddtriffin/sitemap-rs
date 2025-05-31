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
    /// The caption of the image.
    pub caption: Option<String>,
    /// The geographical location of the image, e.g. "Limerick, Ireland".
    pub geo_location: Option<String>,
    /// The title of the image.
    pub title: Option<String>,
    /// A URL to the license of the image.
    pub license: Option<String>,
}

impl Image {
    #[must_use]
    pub fn new(location: String) -> Self {
        Self {
            location,
            caption: None,
            geo_location: None,
            title: None,
            license: None,
        }
    }

    /// Builder methods for optional fields
    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }
    pub fn geo_location(mut self, geo_location: impl Into<String>) -> Self {
        self.geo_location = Some(geo_location.into());
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn license(mut self, license: impl Into<String>) -> Self {
        self.license = Some(license.into());
        self
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

        if let Some(caption) = self.caption {
            let mut caption_el = XMLElement::new("image:caption");
            caption_el.add_text(caption)?;
            image.add_child(caption_el)?;
        }
        if let Some(geo_location) = self.geo_location {
            let mut geo_el = XMLElement::new("image:geo_location");
            geo_el.add_text(geo_location)?;
            image.add_child(geo_el)?;
        }
        if let Some(title) = self.title {
            let mut title_el = XMLElement::new("image:title");
            title_el.add_text(title)?;
            image.add_child(title_el)?;
        }
        if let Some(license) = self.license {
            let mut license_el = XMLElement::new("image:license");
            license_el.add_text(license)?;
            image.add_child(license_el)?;
        }

        Ok(image)
    }
}
