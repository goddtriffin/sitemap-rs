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

### Index Sitemap

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

### Video Sitemap

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

**Project is under active maintenance - even if there are no recent commits! Please submit an issue / bug request if you the library needs updating for any reason!**

Built with: `Rust 1.63`

### Feature Requests

I would love to have this library use [quick-xml](https://github.com/tafia/quick-xml) instead of [xml-builder](https://github.com/cocool97/xml-builder).

The `quick-xml` library is built for speed and it supports not only writing files, but reading them too.
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
