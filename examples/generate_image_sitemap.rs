use sitemap_rs::image::Image;
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;

fn main() {
    let urls: Vec<Url> = vec![
        Url::builder(String::from("https://www.toddgriffin.me/sample1.html"))
            .images(vec![
                Image::new(String::from("https://www.toddgriffin.me/image.jpg")),
                Image::new(String::from("https://www.toddgriffin.me/photo.jpg")),
            ])
            .build()
            .unwrap(),
        Url::builder(String::from("https://www.toddgriffin.me/sample2.html"))
            .images(vec![Image::new(String::from(
                "https://www.toddgriffin.me/picture.jpg",
            ))])
            .build()
            .unwrap(),
    ];

    let url_set: UrlSet = UrlSet::new(urls).unwrap();
    let mut buf: Vec<u8> = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
}
