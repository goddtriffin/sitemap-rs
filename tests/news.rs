use chrono::{DateTime, Utc};
use sitemap_rs::news::{News, Publication};

#[test]
fn test_constructor_only_required_fields() {
    let news: News = News::new(
        Publication::new(String::from("The Todd Times"), String::from("en")),
        DateTime::from(Utc::now()),
        String::from(
            "Local Software Engineer, Todd, Finally Completes Project He Has Talked About For Years",
        ),
    );
    assert!(!news.title.is_empty());
}
