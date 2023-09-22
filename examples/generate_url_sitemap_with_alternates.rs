use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::url::{Alternate, ChangeFrequency, Url};
use sitemap_rs::url_set::UrlSet;

fn main() {
    /*
     * There are two functions you can use to add alternates. The first one is
     * alternates() which expects a Vector with Alternate objects in it.
     * alternates() overwrites existing values in the alternates-attribute
     * of Url. The second one is push_alternate() which expects 2 &str, hreflang
     * and href. push_alternate() appends to the alternates-attribute instead of
     * overwriting.
     * In the following example both are used.
     */
    let urls: Vec<Url> = vec![Url::builder(String::from("https://www.example.com/"))
        .last_modified(DateTime::from_utc(
            NaiveDate::from_ymd_opt(2005, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            FixedOffset::east_opt(0).unwrap(),
        ))
        .change_frequency(ChangeFrequency::Monthly)
        .priority(0.8)
        .alternates(vec![Alternate {
            hreflang: String::from("en-US"),
            href: String::from("https://www.example.com/"),
        }])
        .push_alternate(
            String::from("de-DE"),
            String::from("https://de.example.com/"),
        )
        .push_alternate(
            String::from("de-CH"),
            String::from("https://ch.example.com/de"),
        )
        .push_alternate(
            String::from("fr-CH"),
            String::from("https://ch.example.com/de"),
        )
        .push_alternate(String::from("it"), String::from("https://it.example.com/"))
        .push_alternate(
            String::from("x-default"),
            String::from("https://www.example.com/country-selector"),
        )
        .build()
        .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();
}
