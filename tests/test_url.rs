extern crate core;

use chrono::{DateTime, Utc};
use sitemap_rs::image::Image;
use sitemap_rs::url::{ChangeFrequency, DEFAULT_PRIORITY, Link, Url};
use sitemap_rs::url_error::UrlError;

#[test]
fn test_constructor_only_required_fields() {
    let url_result: Result<Url, UrlError> = Url::new(
        String::from("https://www.toddgriffin.me/"),
        vec![],
        None,
        None,
        None,
        None,
        None,
        None,
    );
    assert!(url_result.is_ok());
}

#[test]
fn test_constructor_all_normal_fields() {
    let url_result: Result<Url, UrlError> = Url::new(
        String::from("https://www.toddgriffin.me/"),
        vec![Link::new(
            "de".to_owned(),
            "https://www.toddgriffin.me/de".to_owned(),
        )],
        Some(DateTime::from(Utc::now())),
        Some(ChangeFrequency::Weekly),
        Some(DEFAULT_PRIORITY),
        None,
        None,
        None,
    );
    assert!(url_result.is_ok());
}

#[test]
fn test_constructor_location_too_long() {
    // generate too long location
    let location: String = "t".repeat(2048);

    let url_result: Result<Url, UrlError> =
        Url::new(location, vec![], None, None, None, None, None, None);
    match url_result {
        Ok(_) => panic!("Returned a URL!"),
        Err(e) => match e {
            UrlError::LocationTooLong(location) => assert_eq!(location, "t".repeat(2048)),
            UrlError::PriorityTooLow(_) => panic!("Returned PriorityTooLow!"),
            UrlError::PriorityTooHigh(_) => panic!("Returned PriorityTooHigh!"),
            UrlError::TooManyImages(_) => panic!("Returned TooManyImages!"),
        },
    }
}

#[test]
fn test_constructor_priority_too_low() {
    let url_result: Result<Url, UrlError> = Url::new(
        String::from("https://www.toddgriffin.me/"),
        vec![],
        None,
        None,
        Some(-1.0),
        None,
        None,
        None,
    );
    match url_result {
        Ok(_) => panic!("Returned a URL!"),
        Err(e) => match e {
            UrlError::LocationTooLong(_) => panic!("Returned LocationTooLong!"),
            UrlError::PriorityTooLow(priority) => {
                let expected_priority: f32 = -1.0;
                assert!((priority - expected_priority).abs() < f32::EPSILON);
            }
            UrlError::PriorityTooHigh(_) => panic!("Returned PriorityTooHigh!"),
            UrlError::TooManyImages(_) => panic!("Returned TooManyImages!"),
        },
    }
}

#[test]
fn test_constructor_priority_too_high() {
    let url_result: Result<Url, UrlError> = Url::new(
        String::from("https://www.toddgriffin.me/"),
        vec![],
        None,
        None,
        Some(4.69),
        None,
        None,
        None,
    );
    match url_result {
        Ok(_) => panic!("Returned a URL!"),
        Err(e) => match e {
            UrlError::LocationTooLong(_) => panic!("Returned LocationTooLong!"),
            UrlError::PriorityTooLow(_) => panic!("Returned PriorityTooLow!"),
            UrlError::PriorityTooHigh(priority) => {
                let expected_priority: f32 = 4.69;
                assert!((priority - expected_priority).abs() < f32::EPSILON);
            }
            UrlError::TooManyImages(_) => panic!("Returned TooManyImages!"),
        },
    }
}

#[test]
fn test_constructor_too_many_images() {
    // generate over 1,000 images
    let mut images: Vec<Image> = vec![];
    for _ in 0..1001 {
        let image: Image = Image::new(String::from("https://www.toddgriffin.me/image.webp"));
        images.push(image);
    }

    let url_result: Result<Url, UrlError> = Url::new(
        String::from("https://www.toddgriffin.me/"),
        vec![],
        None,
        None,
        None,
        Some(images),
        None,
        None,
    );
    match url_result {
        Ok(_) => panic!("Returned a URL!"),
        Err(e) => match e {
            UrlError::LocationTooLong(_) => panic!("Returned LocationTooLong!"),
            UrlError::PriorityTooLow(_) => panic!("Returned PriorityTooLow!"),
            UrlError::PriorityTooHigh(_) => panic!("Returned PriorityTooHigh!"),
            UrlError::TooManyImages(count) => assert_eq!(1001, count),
        },
    }
}
