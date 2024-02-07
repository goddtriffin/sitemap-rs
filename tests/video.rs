use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::video::{Platform, PlatformType, Relationship, Restriction, Uploader, Video};
use sitemap_rs::video_error::VideoError;
use std::collections::HashSet;

#[test]
fn test_constructor_only_required_fields() {
    let video_result: Result<Video, VideoError> = Video::new(
        String::from("http://www.example.com/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        String::from("Alkis shows you how to get perfectly done steaks every time"),
        String::from("http://streamserver.example.com/video123.mp4"),
        String::from("http://www.example.com/videoplayer.php?video=123"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
    assert!(video_result.is_ok());
}

#[test]
fn test_constructor_all_fields() {
    let video_result: Result<Video, VideoError> = Video::new(
        String::from("http://www.example.com/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        String::from("Alkis shows you how to get perfectly done steaks every time"),
        String::from("http://streamserver.example.com/video123.mp4"),
        String::from("http://www.example.com/videoplayer.php?video=123"),
        Some(600),
        Some(DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2021, 11, 5)
                .unwrap()
                .and_hms_opt(11, 20, 30)
                .unwrap(),
            FixedOffset::east_opt(8 * 3600).unwrap(),
        )),
        Some(4.2),
        Some(12345),
        Some(DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2007, 11, 5)
                .unwrap()
                .and_hms_opt(11, 20, 30)
                .unwrap(),
            FixedOffset::east_opt(8 * 3600).unwrap(),
        )),
        Some(true),
        Some(Restriction::new(
            HashSet::from([
                String::from("IE"),
                String::from("GB"),
                String::from("US"),
                String::from("CA"),
            ]),
            Relationship::Allow,
        )),
        Some(Platform::new(
            HashSet::from([PlatformType::Web, PlatformType::Tv]),
            Relationship::Allow,
        )),
        Some(true),
        Some(Uploader::new(
            String::from("GrillyMcGrillserson"),
            Some(String::from(
                "http://www.example.com/users/grillymcgrillerson",
            )),
        )),
        Some(false),
        Some(vec![
            String::from("steak"),
            String::from("meat"),
            String::from("summer"),
            String::from("outdoor"),
        ]),
    );
    assert!(video_result.is_ok());
}

#[test]
fn test_constructor_description_too_long() {
    let mut description: String = String::new();
    for _ in 0..2049 {
        description.push('5');
    }

    let video_result: Result<Video, VideoError> = Video::new(
        String::from("http://www.example.com/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        description,
        String::from("http://streamserver.example.com/video123.mp4"),
        String::from("http://www.example.com/videoplayer.php?video=123"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    match video_result {
        Ok(_) => panic!("Returned a Video!"),
        Err(e) => match e {
            VideoError::DescriptionTooLong(count) => assert_eq!(2049, count),
            VideoError::DurationTooShort(_) => panic!("Returned DurationTooShort!"),
            VideoError::DurationTooLong(_) => panic!("Returned DurationTooLong!"),
            VideoError::RatingTooLow(_) => panic!("Returned RatingTooLow!"),
            VideoError::RatingTooHigh(_) => panic!("Returned RatingTooHigh!"),
            VideoError::UploaderNameTooLong(_) => panic!("Returned UploaderNameTooLong!"),
            VideoError::TooManyTags(_) => panic!("Returned TooManyTags!"),
        },
    }
}

#[test]
fn test_constructor_duration_too_short() {
    let video_result: Result<Video, VideoError> = Video::new(
        String::from("http://www.example.com/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        String::from("Alkis shows you how to get perfectly done steaks every time"),
        String::from("http://streamserver.example.com/video123.mp4"),
        String::from("http://www.example.com/videoplayer.php?video=123"),
        Some(0),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    match video_result {
        Ok(_) => panic!("Returned a Video!"),
        Err(e) => match e {
            VideoError::DescriptionTooLong(_) => panic!("Returned DescriptionTooLong!"),
            VideoError::DurationTooShort(duration) => assert_eq!(0, duration),
            VideoError::DurationTooLong(_) => panic!("Returned DurationTooLong!"),
            VideoError::RatingTooLow(_) => panic!("Returned RatingTooLow!"),
            VideoError::RatingTooHigh(_) => panic!("Returned RatingTooHigh!"),
            VideoError::UploaderNameTooLong(_) => panic!("Returned UploaderNameTooLong!"),
            VideoError::TooManyTags(_) => panic!("Returned TooManyTags!"),
        },
    }
}

#[test]
fn test_constructor_duration_too_long() {
    let video_result: Result<Video, VideoError> = Video::new(
        String::from("http://www.example.com/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        String::from("Alkis shows you how to get perfectly done steaks every time"),
        String::from("http://streamserver.example.com/video123.mp4"),
        String::from("http://www.example.com/videoplayer.php?video=123"),
        Some(28_801),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    match video_result {
        Ok(_) => panic!("Returned a Video!"),
        Err(e) => match e {
            VideoError::DescriptionTooLong(_) => panic!("Returned DescriptionTooLong!"),
            VideoError::DurationTooShort(_) => panic!("Returned DurationTooShort!"),
            VideoError::DurationTooLong(duration) => assert_eq!(28_801, duration),
            VideoError::RatingTooLow(_) => panic!("Returned RatingTooLow!"),
            VideoError::RatingTooHigh(_) => panic!("Returned RatingTooHigh!"),
            VideoError::UploaderNameTooLong(_) => panic!("Returned UploaderNameTooLong!"),
            VideoError::TooManyTags(_) => panic!("Returned TooManyTags!"),
        },
    }
}

#[test]
fn test_constructor_rating_too_low() {
    let video_result: Result<Video, VideoError> = Video::new(
        String::from("http://www.example.com/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        String::from("Alkis shows you how to get perfectly done steaks every time"),
        String::from("http://streamserver.example.com/video123.mp4"),
        String::from("http://www.example.com/videoplayer.php?video=123"),
        None,
        None,
        Some(-1.0),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    match video_result {
        Ok(_) => panic!("Returned a Video!"),
        Err(e) => match e {
            VideoError::DescriptionTooLong(_) => panic!("Returned DescriptionTooLong!"),
            VideoError::DurationTooShort(_) => panic!("Returned DurationTooShort!"),
            VideoError::DurationTooLong(_) => panic!("Returned DurationTooLong!"),
            VideoError::RatingTooLow(rating) => {
                let expected_rating: f32 = -1.0;
                assert!((rating - expected_rating).abs() < f32::EPSILON);
            }
            VideoError::RatingTooHigh(_) => panic!("Returned RatingTooHigh!"),
            VideoError::UploaderNameTooLong(_) => panic!("Returned UploaderNameTooLong!"),
            VideoError::TooManyTags(_) => panic!("Returned TooManyTags!"),
        },
    }
}

#[test]
fn test_constructor_rating_too_high() {
    let video_result: Result<Video, VideoError> = Video::new(
        String::from("http://www.example.com/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        String::from("Alkis shows you how to get perfectly done steaks every time"),
        String::from("http://streamserver.example.com/video123.mp4"),
        String::from("http://www.example.com/videoplayer.php?video=123"),
        None,
        None,
        Some(6.69),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    match video_result {
        Ok(_) => panic!("Returned a Video!"),
        Err(e) => match e {
            VideoError::DescriptionTooLong(_) => panic!("Returned DescriptionTooLong!"),
            VideoError::DurationTooShort(_) => panic!("Returned DurationTooShort!"),
            VideoError::DurationTooLong(_) => panic!("Returned DurationTooLong!"),
            VideoError::RatingTooLow(_) => panic!("Returned RatingTooLow!"),
            VideoError::RatingTooHigh(rating) => {
                let expected_rating: f32 = 6.69;
                assert!((rating - expected_rating).abs() < f32::EPSILON);
            }
            VideoError::UploaderNameTooLong(_) => panic!("Returned UploaderNameTooLong!"),
            VideoError::TooManyTags(_) => panic!("Returned TooManyTags!"),
        },
    }
}

#[test]
fn test_constructor_uploader_name_too_long() {
    let mut uploader_name: String = String::new();
    for _ in 0..256 {
        uploader_name.push('5');
    }
    let uploader: Uploader = Uploader::new(uploader_name, None);

    let video_result: Result<Video, VideoError> = Video::new(
        String::from("http://www.example.com/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        String::from("Alkis shows you how to get perfectly done steaks every time"),
        String::from("http://streamserver.example.com/video123.mp4"),
        String::from("http://www.example.com/videoplayer.php?video=123"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(uploader),
        None,
        None,
    );

    match video_result {
        Ok(_) => panic!("Returned a Video!"),
        Err(e) => match e {
            VideoError::DescriptionTooLong(_) => panic!("Returned DescriptionTooLong!"),
            VideoError::DurationTooShort(_) => panic!("Returned DurationTooShort!"),
            VideoError::DurationTooLong(_) => panic!("Returned DurationTooLong!"),
            VideoError::RatingTooLow(_) => panic!("Returned RatingTooLow!"),
            VideoError::RatingTooHigh(_) => panic!("Returned RatingTooHigh!"),
            VideoError::UploaderNameTooLong(count) => assert_eq!(256, count),
            VideoError::TooManyTags(_) => panic!("Returned TooManyTags!"),
        },
    }
}

#[test]
fn test_constructor_too_many_tags() {
    let mut tags: Vec<String> = vec![];
    for i in 0..33 {
        tags.push(format!("tag_{i}"));
    }

    let video_result: Result<Video, VideoError> = Video::new(
        String::from("http://www.example.com/thumbs/123.jpg"),
        String::from("Grilling steaks for summer"),
        String::from("Alkis shows you how to get perfectly done steaks every time"),
        String::from("http://streamserver.example.com/video123.mp4"),
        String::from("http://www.example.com/videoplayer.php?video=123"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(tags),
    );

    match video_result {
        Ok(_) => panic!("Returned a Video!"),
        Err(e) => match e {
            VideoError::DescriptionTooLong(_) => panic!("Returned DescriptionTooLong!"),
            VideoError::DurationTooShort(_) => panic!("Returned DurationTooShort!"),
            VideoError::DurationTooLong(_) => panic!("Returned DurationTooLong!"),
            VideoError::RatingTooLow(_) => panic!("Returned RatingTooLow!"),
            VideoError::RatingTooHigh(_) => panic!("Returned RatingTooHigh!"),
            VideoError::UploaderNameTooLong(_) => panic!("Returned UploaderNameTooLong!"),
            VideoError::TooManyTags(count) => assert_eq!(33, count),
        },
    }
}
