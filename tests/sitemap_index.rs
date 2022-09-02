extern crate core;

use sitemap_rs::sitemap::Sitemap;
use sitemap_rs::sitemap_index::SitemapIndex;
use sitemap_rs::sitemap_index_error::SitemapIndexError;

#[test]
fn test_constructor_only_required_fields() {
    let sitemaps: Vec<Sitemap> = vec![Sitemap::new(
        String::from("https://www.toddgriffin.me/sitemap.xml"),
        None,
    )];

    let sitemap_index: Result<SitemapIndex, SitemapIndexError> = SitemapIndex::new(sitemaps);
    assert!(sitemap_index.is_ok());
}

#[test]
fn test_constructor_too_many_sitemaps() {
    let mut sitemaps: Vec<Sitemap> = vec![];
    for _ in 0..50_001 {
        sitemaps.push(Sitemap::new(
            String::from("https://www.toddgriffin.me/sitemap.xml"),
            None,
        ));
    }

    let sitemap_result: Result<SitemapIndex, SitemapIndexError> = SitemapIndex::new(sitemaps);
    match sitemap_result {
        Ok(_) => panic!("Returned a SitemapIndex!"),
        Err(e) => match e {
            SitemapIndexError::TooManySitemaps(_) => (),
        },
    }
}
