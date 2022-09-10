use sitemap_rs::image::Image;
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;
use std::path::PathBuf;

fn main() {
    let urls: Vec<Url> = vec![
        Url::builder(String::from("http://example.com/sample1.html"))
            .images(vec![
                Image::new(String::from("http://example.com/image.jpg")),
                Image::new(String::from("http://example.com/photo.jpg")),
            ])
            .build()
            .expect("failed a <url> validation"),
        Url::builder(String::from("http://example.com/sample2.html"))
            .images(vec![Image::new(String::from(
                "http://example.com/picture.jpg",
            ))])
            .build()
            .expect("failed a <url> validation"),
    ];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    url_set
        .write_to_file(PathBuf::from("./target/image-sitemap.xml"))
        .unwrap();
}
