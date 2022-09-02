# sitemap-rs

Library to generate URL, Index, Image, Video, and News sitemaps.

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

*These examples should be exactly congruent to the examples found within the online documentation for each sitemap type.*

### URL Sitemap

`cargo run --example generate_url_sitemap`

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

```xml
<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <sitemap>
    <loc>http://www.example.com/sitemap1.xml.gz</loc>
    <lastmod>2004-10-01T18:23:17+00:00</lastmod>
  </sitemap>
  <sitemap>
    <loc>http://www.example.com/sitemap2.xml.gz</loc>
    <lastmod>2005-01-01T00:00:00+00:00</lastmod>
  </sitemap>
</sitemapindex>
```

### Image Sitemap

`cargo run --example generate_image_sitemap`

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

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:video="http://www.google.com/schemas/sitemap-video/1.1">
  <url>
    <loc>http://www.example.com/videos/some_video_landing_page.html</loc>
    <video:video>
      <video:thumbnail_loc>http://www.example.com/thumbs/123.jpg</video:thumbnail_loc>
      <video:title>Grilling steaks for summer</video:title>
      <video:description>Alkis shows you how to get perfectly done steaks every time</video:description>
      <video:content_loc>http://streamserver.example.com/video123.mp4</video:content_loc>
      <video:player_loc>http://www.example.com/videoplayer.php?video=123</video:player_loc>
      <video:duration>600</video:duration>
      <video:expiration_date>2021-11-05T19:20:30+08:00</video:expiration_date>
      <video:rating>4.2</video:rating>
      <video:view_count>12345</video:view_count>
      <video:publication_date>2007-11-05T19:20:30+08:00</video:publication_date>
      <video:family_friendly>yes</video:family_friendly>
      <video:restriction relationship="allow">IE GB US CA</video:restriction>
      <video:platform relationship="allow">web tv</video:platform>
      <video:requires_subscription>yes</video:requires_subscription>
      <video:uploader info="http://www.example.com/users/grillymcgrillerson">GrillyMcGrillserson</video:uploader>
      <video:live>no</video:live>
      <video:tag>steak</video:tag>
      <video:tag>meat</video:tag>
      <video:tag>summer</video:tag>
      <video:tag>outdoor</video:tag>
    </video:video>
  </url>
</urlset>
```

### News Sitemap

`cargo run --example generate_news_sitemap`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:news="http://www.google.com/schemas/sitemap-news/0.9">
  <url>
    <loc>http://www.example.org/business/article55.html</loc>
    <news:news>
      <news:publication>
        <news:name>The Example Times</news:name>
        <news:language>en</news:language>
      </news:publication>
      <news:publication_date>2008-12-23T09:10:11+00:00</news:publication_date>
      <news:title>Companies A, B in Merger Talks</news:title>
    </news:news>
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

#### Reading sitemap files (+ possible speed boost)

I would love to have this library use [quick-xml](https://github.com/tafia/quick-xml) instead of [xml-builder](https://github.com/cocool97/xml-builder).

The `quick-xml` library is built for speed and supports not only writing files, but reading them too.
I haven't benchmarked `xml-builder` or its use in this library, so I cannot state the impact `quick-xml` will have there.

I originally went with `xml-builder` due to how extremely easy it is to learn and use.
It is by far fast enough for my use-cases, so I didn't have to reach for anything else.

If you like what this library provides, but simply need the ability to parse sitemaps and could also use a speed boost - 
please consider pushing a pull request! (Preferably one that replaces `xml-builder` with `quick-xml` lol)

#### Builder pattern

Currently, the only way to create instances of each struct is by using `::new()`.
By implementing the [Builder pattern](https://en.wikipedia.org/wiki/Builder_pattern) on each struct, less work has to be done
on the library users' side as they don't have to throw in many `None` values for each optional field they don't want to use.
Not only would this make the library more ergonomic to use, but it would vastly improve readability (specifically at 
each struct initialization point).

This hasn't been prioritized yet as I am currently satisfied with `::new()` for my use cases.
Pull requests are welcome!

#### Codified country codes

In video sitemaps, there is a tag called `<video: restriction>` where the text is a space-delimited list of country codes
in [ISO 3166 format](https://en.wikipedia.org/wiki/ISO_3166).

Currently, the country codes are typed-hinted as merely a `HashSet<String>`.
It would be awesome if there was an enum/struct that codified each ISO 3166 country code as a separate entity, so this 
library could have extra assurances that each code was valid.

The [isocountry-rs](https://github.com/sifton/isocountry-rs) and 
[rust_iso_3166](https://github.com/rust-iso/rust_iso3166) libraries looks promising.

This hasn't been prioritized yet as I am currently satisfied with `HashSet<String>` for my use cases.
Pull requests are welcome!

### Commands

- `make lint`
  - Lints the codebase via `cargo fmt`.
- `make test`
  - Tests the codebase via:
    - `cargo fmt`
    - `cargo check`
    - `cargo clippy` (with insanely strict defaults)
    - `cargo test`.
