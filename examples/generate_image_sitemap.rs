use sitemap_rs::image::Image;
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;

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
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
}
