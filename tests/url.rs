extern crate core;

use chrono::{DateTime, Utc};
use sitemap_rs::image::Image;
use sitemap_rs::url::{Alternate, ChangeFrequency, Url, DEFAULT_PRIORITY};
use sitemap_rs::url_error::UrlError;

#[test]
fn test_constructor_only_required_fields() {
    let url_result: Result<Url, UrlError> = Url::new(
        String::from("https://www.toddgriffin.me/"),
        None,
        None,
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
        Some(DateTime::from(Utc::now())),
        Some(ChangeFrequency::Weekly),
        Some(DEFAULT_PRIORITY),
        None,
        None,
        None,
        None,
        None,
    );
    assert!(url_result.is_ok());
}

#[test]
fn test_constructor_priority_too_low() {
    let url_result: Result<Url, UrlError> = Url::new(
        String::from("https://www.toddgriffin.me/"),
        Some(DateTime::from(Utc::now())),
        Some(ChangeFrequency::Weekly),
        Some(-1.0),
        None,
        None,
        None,
        None,
        None,
    );
    match url_result {
        Ok(_) => panic!("Returned a URL!"),
        Err(e) => match e {
            UrlError::PriorityTooLow(priority) => {
                let expected_priority: f32 = -1.0;
                assert!((priority - expected_priority).abs() < f32::EPSILON);
            }
            UrlError::PriorityTooHigh(_) => panic!("Returned PriorityTooHigh!"),
            UrlError::TooManyImages(_) => panic!("Returned TooManyImages!"),
            UrlError::DuplicateAlternateHreflangs(..) => {
                panic!("Returned DuplicateAlternateHreflangs!");
            }
        },
    }
}

#[test]
fn test_constructor_priority_too_high() {
    let url_result: Result<Url, UrlError> = Url::new(
        String::from("https://www.toddgriffin.me/"),
        Some(DateTime::from(Utc::now())),
        Some(ChangeFrequency::Weekly),
        Some(4.69),
        None,
        None,
        None,
        None,
        None,
    );
    match url_result {
        Ok(_) => panic!("Returned a URL!"),
        Err(e) => match e {
            UrlError::PriorityTooLow(_) => panic!("Returned PriorityTooLow!"),
            UrlError::PriorityTooHigh(priority) => {
                let expected_priority: f32 = 4.69;
                assert!((priority - expected_priority).abs() < f32::EPSILON);
            }
            UrlError::TooManyImages(_) => panic!("Returned TooManyImages!"),
            UrlError::DuplicateAlternateHreflangs(..) => {
                panic!("Returned DuplicateAlternateHreflangs!");
            }
        },
    }
}

#[test]
fn test_constructor_too_many_images() {
    // generate over 1,000 images
    let mut images: Vec<Image> = vec![];
    for _ in 0..1001 {
        let image: Image = Image::new(String::from(
            "https://www.toddgriffin.me/static/image/social/profile-picture.webp",
        ));
        images.push(image);
    }

    let url_result: Result<Url, UrlError> = Url::new(
        String::from("https://www.toddgriffin.me/"),
        Some(DateTime::from(Utc::now())),
        Some(ChangeFrequency::Weekly),
        Some(DEFAULT_PRIORITY),
        Some(images),
        None,
        None,
        None,
        None,
    );
    match url_result {
        Ok(_) => panic!("Returned a URL!"),
        Err(e) => match e {
            UrlError::PriorityTooLow(_) => panic!("Returned PriorityTooLow!"),
            UrlError::PriorityTooHigh(_) => panic!("Returned PriorityTooHigh!"),
            UrlError::TooManyImages(count) => assert_eq!(1001, count),
            UrlError::DuplicateAlternateHreflangs(..) => {
                panic!("Returned DuplicateAlternateHreflangs!");
            }
        },
    }
}

#[test]
fn test_constructor_duplicate_alternate_hreflangs() {
    let alternates: Vec<Alternate> = vec![
        Alternate {
            hreflang: String::from("en-US"),
            href: String::from("https://www.example.com/"),
        },
        Alternate {
            hreflang: String::from("en-US"),
            href: String::from("https://www.example.com/"),
        },
    ];

    let url_result: Result<Url, UrlError> = Url::new(
        String::from("https://www.example.com/"),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(alternates),
        None,
    );
    match url_result {
        Ok(_) => panic!("Returned a URL!"),
        Err(e) => match e {
            UrlError::PriorityTooLow(_) => panic!("Returned PriorityTooLow!"),
            UrlError::PriorityTooHigh(_) => panic!("Returned PriorityTooHigh!"),
            UrlError::TooManyImages(_) => panic!("Returned TooManyImages!"),
            UrlError::DuplicateAlternateHreflangs(hreflang, href) => {
                assert_eq!(String::from("en-US"), hreflang);
                assert_eq!(String::from("https://www.example.com/"), href);
            }
        },
    }
}
