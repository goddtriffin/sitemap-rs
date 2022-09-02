# sitemap-rs

## Features

### Generates sitemaps

- [URL sitemaps](https://www.sitemaps.org/protocol.html)
- [Index sitemaps](https://www.sitemaps.org/protocol.html)
- [Image sitemaps](https://developers.google.com/search/docs/advanced/sitemaps/image-sitemaps)
- [Video sitemaps](https://developers.google.com/search/docs/advanced/sitemaps/video-sitemaps)
- [News sitemaps](https://developers.google.com/search/docs/advanced/sitemaps/news-sitemap)

### Validates sitemap data

There are a bunch of restrictions as to what data your sitemaps can hold.
This library surfaces these validation issues at struct instantiation time.
Now you don't have to wait for [Google Search Console](https://search.google.com/search-console/about) 
or [Bing Webmaster Tools](https://www.bing.com/webmasters/tools) to alert you of sitemap issues before you can fix data
problems.

#### Validations

- URL Sitemap
  - `TooManyUrls`
    - Can only contain as many as `50,000` `<url>`.
  - `TooMuchNews`
    - Can only contain as many as `1,000` `<news: news>`.
  - `PriorityTooLow` and `PriorityTooHigh`
    - A `<priority>` must be between `0.0` and `1.0` (inclusive).
  - `TooManyImages`
    - Can only contain as many as `1,000` `<image: image>`.
- Index Sitemap
  - `TooManySitemaps`
    - Can only contain as many as `50,000` `<sitemap>`.
- Video Sitemap
  - `DescriptionTooLong`
    - A `<description>` must be no longer than `2048` characters.
  - `DurationTooShort` and `DurationTooLong`
    - A `<duration>` must be between `1` and `28,800` seconds (inclusive).
  - `RatingTooLow` and `RatingTooHigh`
    - A `<rating>` must be between `0.0` and `5.0` (inclusive).
  - `UploaderNameTooLong`
    - An `<uploader>`'s `<name>` must be no longer than `255` characters.
  - `TooManyTags`
    - Must contain no more than `32` `<tag>`.

## Restrictions

The library **cannot** parse sitemaps of any kind.

## Examples

### URL Sitemap

`cargo run --example generate_url_sitemap`

```rust
fn main() {
  let url: Url = Url::new(
    String::from("http://www.example.com/"),
    Some(DateTime::from_utc(
      NaiveDate::from_ymd(2005, 1, 1).and_hms(9, 10, 11),
      FixedOffset::east(0),
    )),
    Some(ChangeFrequency::Monthly),
    Some(0.8),
    None,
    None,
    None,
  )
  .expect("failed a <url> validation");

  let url_set: UrlSet = UrlSet::new(vec![url]).expect("failed a <urlset> validation");
  url_set
    .write_to_file(PathBuf::from("./target/url-sitemap.xml"))
    .unwrap();
}
```

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>http://www.example.com/</loc>
    <lastmod>2005-01-01T09:10:11+00:00</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
</urlset>
```

### Index Sitemap

`cargo run --example generate_index_sitemap`

```rust
println!("Hello world!");
```

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>https://www.toddgriffin.me/</loc>
        <lastmod>2022-07-28T19:11:34Z</lastmod>
        <changefreq>weekly</changefreq>
        <priority>0.5</priority>
    </url>
</urlset>
```

### Image Sitemap

`cargo run --example generate_image_sitemap`

```rust
fn main() {
  let urls: Vec<Url> = vec![
    Url::new(
      String::from("http://example.com/sample1.html"),
      None,
      None,
      None,
      Some(vec![
        Image::new(String::from("http://example.com/image.jpg")),
        Image::new(String::from("http://example.com/photo.jpg")),
      ]),
      None,
      None,
    )
    .expect("failed a <url> validation"),
    Url::new(
      String::from("http://example.com/sample2.html"),
      None,
      None,
      None,
      Some(vec![Image::new(String::from(
        "http://example.com/picture.jpg",
      ))]),
      None,
      None,
    )
    .expect("failed a <url> validation"),
  ];

  let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
  url_set
    .write_to_file(PathBuf::from("./target/image-sitemap.xml"))
    .unwrap();
}
```

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:image="http://www.google.com/schemas/sitemap-image/1.1">
  <url>
    <loc>http://example.com/sample1.html</loc>
    <image:image>
      <image:loc>http://example.com/image.jpg</image:loc>
    </image:image>
    <image:image>
      <image:loc>http://example.com/photo.jpg</image:loc>
    </image:image>
  </url>
  <url>
    <loc>http://example.com/sample2.html</loc>
    <image:image>
      <image:loc>http://example.com/picture.jpg</image:loc>
    </image:image>
  </url>
</urlset>
```

### Video Sitemap

`cargo run --example generate_video_sitemap`

```rust
println!("Hello world!");
```

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>https://www.toddgriffin.me/</loc>
        <lastmod>2022-07-28T19:11:34Z</lastmod>
        <changefreq>weekly</changefreq>
        <priority>0.5</priority>
    </url>
</urlset>
```

### News Sitemap

`cargo run --example generate_news_sitemap`

```rust
println!("Hello world!");
```

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>https://www.toddgriffin.me/</loc>
        <lastmod>2022-07-28T19:11:34Z</lastmod>
        <changefreq>weekly</changefreq>
        <priority>0.5</priority>
    </url>
</urlset>
```

## Alternative libraries

*The `rust-sitemap` and `sitewriter` libraries are by far the best alternatives.*

*This pro/con list is accurate as of the most recent update to this document.*

### [rust-sitemap](https://github.com/svmk/rust-sitemap)

#### Pros:
  - Supports URL, Index sitemaps
  - Supports reading files
  - Supports writing files

#### Cons:
  - Doesn't support Image, Video, News sitemaps
  - Only supports some validations
  - Low struct/method documentation

### [sitewriter](https://github.com/edg-l/sitewriter)

#### Pros:
  - Supports URL sitemaps
  - Supports writing files
  - Support builder pattern
  - uses [quick-xml](https://github.com/tafia/quick-xml), so it should be quite fast
  - Some struct/method documentation

#### Cons:
  - Doesn't support Index, Image, Video, News sitemaps
  - Doesn't support reading files
  - Zero data validations

### [sitemap-iter](https://github.com/Icelk/sitemap-iter/)

#### Pros:
  - Supports URL sitemaps
  - Supports reading files

#### Cons:
  - Doesn't support Index, Image, Video, News sitemaps
  - Doesn't support writing files
  - Zero data validations
  - Low struct/method documentation

### [rust-sitemap-writer](https://github.com/uiuifree/rust-sitemap-writer)

#### Pros:
  - Supports URL sitemaps
  - Supports writing files

#### Cons:
  - Doesn't support Index, Image, Video, News sitemaps
  - Doesn't support reading files
  - Zero data validations
  - Zero struct/method documentation

### [mdbook-sitemap-generator](https://github.com/rxdn/mdbook-sitemap-generator)

#### Pros:
  - Semi-supports URL sitemaps
  - Supports writing files

#### Cons:
  - Not a general use sitemap library
  - Doesn't support every possible tag of URL sitemaps
  - Doesn't support Index, Image, Video, News sitemaps
  - Doesn't support reading files
  - Zero data validations
  - Zero struct/method documentation

## Developers

**Project is under active maintenance - even if there are no recent commits! Please submit an issue / bug request if the library needs updating for any reason!**

Built with: `Rust 1.63`

### Feature Requests

I would love to have this library use [quick-xml](https://github.com/tafia/quick-xml) instead of [xml-builder](https://github.com/cocool97/xml-builder).

The `quick-xml` library is built for speed and supports not only writing files, but reading them too.
I haven't benchmarked `xml-builder` or its use in this library, so I cannot state the impact `quick-xml` will have there.

I originally went with `xml-builder` due to how extremely easy it is to learn and use.
It is by far fast enough for my use-cases, so I didn't have to reach for anything else.

If you like what this library provides, but simply need the ability to parse sitemaps and could also use a speed boost - please consider pushing me a pull request!
(Preferably one that replaces `xml-builder` with `quick-xml` lol)

### Commands

- `make lint`
  - Lints the codebase via `cargo fmt`.
- `make test`
  - Tests the codebase via:
    - `cargo fmt`
    - `cargo check`
    - `cargo clippy` (with insanely strict defaults)
    - `cargo test`.
