use sitemap_rs::image::Image;

#[test]
fn test_constructor_only_required_fields() {
    let image: Image = Image::new(String::from(
        "https://www.toddgriffin.me/static/image/social/profile-picture.webp",
    ));
    assert!(!image.location.is_empty());
}

#[test]
fn test_all_fields() {
    let image = Image::new("https://example.com/image.jpg".to_owned())
        .caption("A beautiful view".to_owned())
        .geo_location("New York, USA".to_owned())
        .title("Skyline".to_owned())
        .license("https://example.com/license".to_owned());
    assert_eq!(image.caption.as_deref(), Some("A beautiful view"));
    assert_eq!(image.geo_location.as_deref(), Some("New York, USA"));
    assert_eq!(image.title.as_deref(), Some("Skyline"));
    assert_eq!(
        image.license.as_deref(),
        Some("https://example.com/license")
    );
}
