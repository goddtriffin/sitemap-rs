use chrono::{DateTime, Utc};
use sitemap_rs::news::{News, Publication};
use sitemap_rs::url::Url;
use sitemap_rs::url_set::UrlSet;
use sitemap_rs::url_set_error::UrlSetError;

#[test]
fn test_constructor_only_required_fields() {
    let urls: Vec<Url> = vec![Url::new(
        String::from("https://www.toddgriffin.me/"),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .expect("failed a <url> validation")];

    let url_set_result: Result<UrlSet, UrlSetError> = UrlSet::new(urls);
    assert!(url_set_result.is_ok());
}

#[test]
fn test_constructor_too_many_urls() {
    let mut urls: Vec<Url> = vec![];
    for _ in 0..50_001 {
        let url: Url = Url::new(
            String::from("https://www.toddgriffin.me/"),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .expect("failed a <url> validation");
        urls.push(url);
    }

    let url_set_result: Result<UrlSet, UrlSetError> = UrlSet::new(urls);
    match url_set_result {
        Ok(_) => panic!("Returned a UrlSet!"),
        Err(e) => match e {
            UrlSetError::TooManyUrls(count) => assert_eq!(50_001, count),
            UrlSetError::TooMuchNews(_) => panic!("Returned TooMuchNews!"),
        },
    }
}

#[test]
fn test_constructor_too_much_news() {
    let news: News = News::new(
        Publication::new(
            String::from("The Todd Times"),
            String::from("en"),
        ),
        DateTime::from(Utc::now()),
        String::from("Local Software Engineer, Todd, Finally Completes Project He Has Talked About For Years")
    );

    let mut urls: Vec<Url> = vec![];
    for _ in 0..1001 {
        let url: Url = Url::new(
            String::from("https://www.toddgriffin.me/"),
            None,
            None,
            None,
            None,
            None,
            Some(news.clone()),
        )
        .expect("failed a <url> validation");
        urls.push(url);
    }

    let url_set_result: Result<UrlSet, UrlSetError> = UrlSet::new(urls);
    match url_set_result {
        Ok(_) => panic!("Returned a UrlSet!"),
        Err(e) => match e {
            UrlSetError::TooManyUrls(_) => panic!("Returned TooManyUrls!"),
            UrlSetError::TooMuchNews(count) => assert_eq!(1001, count),
        },
    }
}
