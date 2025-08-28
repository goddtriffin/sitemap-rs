use sitemap_rs::image::Image;

#[test]
fn test_constructor_only_required_fields() {
    let image: Image = Image::new(String::from("https://www.toddgriffin.me/image.webp"));
    assert!(!image.location.is_empty());
}
