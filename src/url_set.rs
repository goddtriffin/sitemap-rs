use crate::url::Url;
use crate::url_set_error::UrlSetError;
use crate::{ENCODING, IMAGE_NAMESPACE, NAMESPACE, NEWS_NAMESPACE, VIDEO_NAMESPACE};
use std::io::Write;
use xml_builder::{XMLBuilder, XMLElement, XMLError, XMLVersion, XML};

/// Encapsulates the file and references the current protocol standard.
pub struct UrlSet {
    /// The XML version.
    pub xml_version: XMLVersion,

    /// The XML encoding.
    pub xml_encoding: String,

    /// The namespace for the \<urlset\>.
    pub xmlns: String,

    /// A namespace extension for allowing \<image\> in the UrlSet.
    pub xmlns_image: Option<String>,

    /// A namespace extension for allowing \<video\> in the UrlSet.
    pub xmlns_video: Option<String>,

    /// A namespace extension for allowing \<new\> in the UrlSet.
    pub xmlns_news: Option<String>,

    /// All the URLs that will become indexed.
    pub urls: Vec<Url>,
}

impl UrlSet {
    /// # Errors
    ///
    /// Will return `UrlSetError::TooManyUrls` if the length of `urls` is above `50,000`.
    pub fn new(urls: Vec<Url>) -> Result<Self, UrlSetError> {
        // UrlSets cannot contain more than 50,000 URLs
        if urls.len() > 50_000 {
            return Err(UrlSetError::TooManyUrls(urls.len()));
        }

        // check if we even need namespaces for images/videos/news
        let mut xmlns_image: Option<String> = None;
        let mut xmlns_video: Option<String> = None;
        let mut xmlns_news: Option<String> = None;
        let mut news_exists: bool = false;
        for url in &urls {
            // if any <url>s exist that contain an image, set the image namespace
            if let Some(images) = &url.images {
                if !images.is_empty() {
                    xmlns_image = Some(IMAGE_NAMESPACE.to_string());
                }
            }

            // if any <url>s exist that contain a video, set the video namespace
            if let Some(videos) = &url.videos {
                if !videos.is_empty() {
                    xmlns_video = Some(VIDEO_NAMESPACE.to_string());
                }
            }

            // check if any URLs have news
            if url.news.is_some() {
                news_exists = true;
                if xmlns_news.is_none() {
                    xmlns_news = Some(NEWS_NAMESPACE.to_string());
                }
            }
        }

        // cannot have more than 1,000 news URLs in a single UrlSet
        if news_exists && urls.len() > 1000 {
            return Err(UrlSetError::TooMuchNews(urls.len()));
        }

        Ok(Self {
            xml_version: XMLVersion::XML1_0,
            xml_encoding: ENCODING.to_string(),
            xmlns: NAMESPACE.to_string(),
            xmlns_image,
            xmlns_video,
            xmlns_news,
            urls,
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

        // create <urlset>
        let mut urlset = XMLElement::new("urlset");
        urlset.add_attribute("xmlns", self.xmlns.as_str());

        // set image namespace, if it exists
        if let Some(xmlns_image) = self.xmlns_image {
            urlset.add_attribute("xmlns:image", xmlns_image.as_str());
        }

        // set video namespace, if it exists
        if let Some(xmlns_video) = self.xmlns_video {
            urlset.add_attribute("xmlns:video", xmlns_video.as_str());
        }

        // set video namespace, if it exists
        if let Some(xmlns_news) = self.xmlns_news {
            urlset.add_attribute("xmlns:news", xmlns_news.as_str());
        }

        // add each <url>
        for url in self.urls {
            urlset.add_child(url.to_xml()?)?;
        }

        // set root element and we're done!
        xml.set_root_element(urlset);
        Ok(xml)
    }

    /// # Errors
    ///
    /// Will return `XMLError` if there is an IO Error dealing with the
    /// underlying writer or if there is an error generating XML.
    pub fn write<W: Write>(self, writer: W) -> Result<(), XMLError> {
        let xml = self.to_xml()?;
        xml.generate(writer)
    }
}
