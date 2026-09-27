use crate::url::Url;
use crate::url_set_error::UrlSetError;
use crate::xml::write_document;
use crate::{IMAGE_NAMESPACE, NAMESPACE, NEWS_NAMESPACE, VIDEO_NAMESPACE, XHTML_NAMESPACE};
use std::io::{self, Write};

/// Encapsulates the file and references the current protocol standard.
pub struct UrlSet {
    /// The namespace for the \<urlset\>.
    pub xmlns: String,

    /// A namespace extension for allowing \<xhtml:link\> (alternate language links) in the `UrlSet`.
    pub xmlns_xhtml: Option<String>,

    /// A namespace extension for allowing \<image\> in the `UrlSet`.
    pub xmlns_image: Option<String>,

    /// A namespace extension for allowing \<video\> in the `UrlSet`.
    pub xmlns_video: Option<String>,

    /// A namespace extension for allowing \<news\> in the `UrlSet`.
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

        // check if we even need namespaces for alternative language links, images, videos, or news
        let mut xmlns_xhtml: Option<String> = None;
        let mut xmlns_image: Option<String> = None;
        let mut xmlns_video: Option<String> = None;
        let mut xmlns_news: Option<String> = None;
        let mut news_exists: bool = false;
        for url in &urls {
            // if any <url>s exist that contain alternate language links, set the xhtml namespace
            if !url.links.is_empty() {
                xmlns_xhtml = Some(XHTML_NAMESPACE.to_string());
            }

            // if any <url>s exist that contain an image, set the image namespace
            if let Some(images) = &url.images
                && !images.is_empty()
            {
                xmlns_image = Some(IMAGE_NAMESPACE.to_string());
            }

            // if any <url>s exist that contain a video, set the video namespace
            if let Some(videos) = &url.videos
                && !videos.is_empty()
            {
                xmlns_video = Some(VIDEO_NAMESPACE.to_string());
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
            xmlns: NAMESPACE.to_string(),
            xmlns_xhtml,
            xmlns_image,
            xmlns_video,
            xmlns_news,
            urls,
        })
    }

    /// Writes this `UrlSet` as an XML document into `writer`.
    ///
    /// # Errors
    ///
    /// Will return `io::Error` if there is an IO Error dealing with the underlying writer.
    pub fn write<W: Write>(&self, writer: W) -> io::Result<()> {
        write_document(writer, |writer| {
            // create <urlset>
            let mut urlset = writer
                .create_element("urlset")
                .with_attribute(("xmlns", self.xmlns.as_str()));

            // set xhtml namespace, if it exists
            if let Some(xmlns_xhtml) = &self.xmlns_xhtml {
                urlset = urlset.with_attribute(("xmlns:xhtml", xmlns_xhtml.as_str()));
            }

            // set image namespace, if it exists
            if let Some(xmlns_image) = &self.xmlns_image {
                urlset = urlset.with_attribute(("xmlns:image", xmlns_image.as_str()));
            }

            // set video namespace, if it exists
            if let Some(xmlns_video) = &self.xmlns_video {
                urlset = urlset.with_attribute(("xmlns:video", xmlns_video.as_str()));
            }

            // set news namespace, if it exists
            if let Some(xmlns_news) = &self.xmlns_news {
                urlset = urlset.with_attribute(("xmlns:news", xmlns_news.as_str()));
            }

            if self.urls.is_empty() {
                urlset.write_empty()?;
                return Ok(());
            }

            // add each <url>
            urlset.write_inner_content(|writer| {
                for url in &self.urls {
                    url.write_xml(writer)?;
                }
                Ok(())
            })?;
            Ok(())
        })
    }
}
