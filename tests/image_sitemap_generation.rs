use pretty_assertions::assert_eq;
use sitemap_rs::image::Image;
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;

#[test]
fn test_image_sitemap_generation() {
    let urls: Vec<Url> = vec![
        Url::builder(String::from("https://www.example.com/"))
            .images(vec![
                Image::new(String::from("https://www.example.com/image.jpg")),
                Image::new(String::from("https://www.example.com/photo.jpg")),
            ])
            .build()
            .expect("failed a <url> validation"),
        Url::builder(String::from("https://www.example.com/page"))
            .images(vec![Image::new(String::from(
                "https://www.example.com/page-banner.jpg",
            ))])
            .build()
            .expect("failed a <url> validation"),
    ];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    let result = String::from_utf8(buf).unwrap();
    assert_eq!(
        "\
        <?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" xmlns:image=\"http://www.google.com/schemas/sitemap-image/1.1\">\n\
            \t<url>\n\
                \t\t<loc>https://www.example.com/</loc>\n\
                \t\t<image:image>\n\
                    \t\t\t<image:loc>https://www.example.com/image.jpg</image:loc>\n\
                \t\t</image:image>\n\
                \t\t<image:image>\n\
                    \t\t\t<image:loc>https://www.example.com/photo.jpg</image:loc>\n\
                \t\t</image:image>\n\
            \t</url>\n\
            \t<url>\n\
                \t\t<loc>https://www.example.com/page</loc>\n\
                \t\t<image:image>\n\
                    \t\t\t<image:loc>https://www.example.com/page-banner.jpg</image:loc>\n\
                \t\t</image:image>\n\
            \t</url>\n\
        </urlset>\n",
        result
    );
}
