# sitemap-rs

[![Version](https://img.shields.io/crates/v/sitemap-rs)](https://crates.io/crates/sitemap-rs)
[![Docs](https://docs.rs/sitemap-rs/badge.svg)](https://docs.rs/sitemap-rs)

A Rust library to generate URL, Index, Image, Video, and News sitemaps.

## Features

### Generates sitemaps

- [URL sitemaps](https://www.sitemaps.org/protocol.html)
- [Index sitemaps](https://www.sitemaps.org/protocol.html)
- [Image sitemaps](https://developers.google.com/search/docs/advanced/sitemaps/image-sitemaps)
- [Video sitemaps](https://developers.google.com/search/docs/advanced/sitemaps/video-sitemaps)
- [News sitemaps](https://developers.google.com/search/docs/advanced/sitemaps/news-sitemap)

### Validates sitemap data

There are a bunch of restrictions as to what data your sitemaps can hold. This
library surfaces these validation issues at struct instantiation time. Now you
don't have to wait for
[Google Search Console](https://search.google.com/search-console/about) or
[Bing Webmaster Tools](https://www.bing.com/webmasters/tools) to alert you of
sitemap issues before you can fix data problems.

#### Validations

- URL Sitemap
  - `LocationTooLong`
    - A `<loc>` must be less than `2,048` characters.
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

This library **cannot** parse sitemaps of any kind (yet! - pull requests
welcome! See Feature Requests section below).

## Examples

### URL Sitemap

`cargo run --example generate_url_sitemap`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">
	<url>
		<loc>https://www.toddgriffin.me/</loc>
		<xhtml:link rel="alternate" hreflang="de" href="https://www.toddgriffin.me/de" />
		<lastmod>1998-01-15T04:20:00+00:00</lastmod>
		<changefreq>monthly</changefreq>
		<priority>0.69</priority>
	</url>
</urlset>
```

### Index Sitemap

`cargo run --example generate_index_sitemap`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
	<sitemap>
		<loc>https://www.toddgriffin.me/sitemap1.xml.gz</loc>
		<lastmod>1998-01-15T04:20:00+00:00</lastmod>
	</sitemap>
	<sitemap>
		<loc>https://www.toddgriffin.me/sitemap2.xml.gz</loc>
		<lastmod>2000-01-31T04:20:00+00:00</lastmod>
	</sitemap>
</sitemapindex>
```

### Image Sitemap

`cargo run --example generate_image_sitemap`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:image="http://www.google.com/schemas/sitemap-image/1.1">
	<url>
		<loc>https://www.toddgriffin.me/sample1.html</loc>
		<image:image>
			<image:loc>https://www.toddgriffin.me/image.jpg</image:loc>
		</image:image>
		<image:image>
			<image:loc>https://www.toddgriffin.me/photo.jpg</image:loc>
		</image:image>
	</url>
	<url>
		<loc>https://www.toddgriffin.me/sample2.html</loc>
		<image:image>
			<image:loc>https://www.toddgriffin.me/picture.jpg</image:loc>
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
		<loc>https://www.toddgriffin.me/videos/some_video_landing_page.html</loc>
		<video:video>
			<video:thumbnail_loc>https://www.toddgriffin.me/thumbs/123.jpg</video:thumbnail_loc>
			<video:title>Grilling steaks for summer</video:title>
			<video:description>Alkis shows you how to get perfectly done steaks every time</video:description>
			<video:content_loc>https://www.toddgriffin.me/video123.mp4</video:content_loc>
			<video:player_loc>https://www.toddgriffin.me/videoplayer.php?video=123</video:player_loc>
			<video:duration>600</video:duration>
			<video:expiration_date>2021-11-05T19:20:30+08:00</video:expiration_date>
			<video:rating>4.2</video:rating>
			<video:view_count>8633</video:view_count>
			<video:publication_date>1998-01-15T12:20:00+08:00</video:publication_date>
			<video:family_friendly>yes</video:family_friendly>
			<video:restriction relationship="allow">CA GB IE US</video:restriction>
			<video:platform relationship="allow">tv web</video:platform>
			<video:requires_subscription>yes</video:requires_subscription>
			<video:uploader info="https://www.toddgriffin.me/users/grillymcgrillerson">GrillyMcGrillserson</video:uploader>
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
		<loc>https://www.toddgriffin.me/business/article55.html</loc>
		<news:news>
			<news:publication>
				<news:name>The Example Times</news:name>
				<news:language>en</news:language>
			</news:publication>
			<news:publication_date>1998-01-15T04:20:00+00:00</news:publication_date>
			<news:title>Companies A, B in Merger Talks</news:title>
		</news:news>
	</url>
</urlset>
```

## Alternative libraries

_The `rust-sitemap` and `sitewriter` libraries are by far the best
alternatives._

_This pro/con list is accurate as of the most recent update to this document._

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
- uses [quick-xml](https://github.com/tafia/quick-xml), so it should be quite
  fast
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

**Project is under active maintenance - even if there are no recent commits!
Please submit an issue / bug request if the library needs updating for any
reason!**

### Philosophy

This library should be fast, efficient, strictly adhere to the Sitemap
specification, and strictly adhere to Google Search Console's best practices.

A feature request will be accepted if it exists in the specification and if it
is a best practice to use it.

For example, here are some deprecated Image Sitemap fields: `<image:caption>`,
`<image:geo_location>`, `<image:title>`, `<image:license>`. While the
specification technically describes these fields, Google Search Console's best
practices is to omit them.

> Over the years, we introduced a number of tags and tag attributes for Google
> sitemap extensions, specifically the Image and Video extensions.
>
> Most of these tags were added to allow site owners to deliver data more easily
> to Search. Upon evaluating the value of the Google sitemap extension tags, we
> decided to officially deprecate some tags and attributes, and remove them from
> our documentation. The deprecated tags will have no effect on indexing and
> search features after August 6, 2022.
>
> If you are a sitemap plugin developer or manage your own sitemaps, there's no
> immediate action required; you can leave these tags and attributes in place
> without drawbacks. In the future, Search Console may show warnings once these
> updates are included in the next schema versions of the Image and Video
> extensions.

Source:
https://developers.google.com/search/docs/crawling-indexing/sitemaps/image-sitemaps

Any contribution which doesn't follow this philosophy will unfortunately be
closed.

On the flip side, if this library has not implemented any feature of the Sitemap
spec - it must be implemented!

### Specification

- https://www.sitemaps.org/protocol.html
- https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap#xml
- https://developers.google.com/search/docs/specialty/international/localized-versions#sitemap
  - https://developers.google.com/search/docs/specialty/international/localized-versions#all-method-guidelines
  - https://developers.google.com/search/docs/specialty/international/localized-versions#xdefault

### Feature Requests

#### Reading sitemap files (+ possible speed boost)

I would love to have this library use
[quick-xml](https://github.com/tafia/quick-xml) instead of
[xml-builder](https://github.com/cocool97/xml-builder).

The `quick-xml` library is built for speed and supports not only writing files,
but reading them too. I haven't benchmarked `xml-builder` or its use in this
library, so I cannot state the impact `quick-xml` will have there.

I originally went with `xml-builder` due to how extremely easy it is to learn
and use. It is by far fast enough for my use-cases, so I didn't have to reach
for anything else.

If you like what this library provides, but simply need the ability to parse
sitemaps and could also use a speed boost - please consider pushing a pull
request! (Preferably one that replaces `xml-builder` with `quick-xml` lol)

#### Codified country codes

In video sitemaps, there is a tag called `<video: restriction>` where the text
is a space-delimited list of country codes in
[ISO 3166 format](https://en.wikipedia.org/wiki/ISO_3166).

Currently, the country codes are typed-hinted as merely a `HashSet<String>`. It
would be awesome if there was an enum/struct that codified each ISO 3166 country
code as a separate entity, so this library could have extra assurances that each
code was valid.

The [isocountry-rs](https://github.com/sifton/isocountry-rs) and
[rust_iso_3166](https://github.com/rust-iso/rust_iso3166) libraries looks
promising.

This hasn't been prioritized yet as I am currently satisfied with
`HashSet<String>` for my use cases. Pull requests are welcome!

### Commands

- `make lint`
- `make test`
- `make fix`

## Credits

Made by [Todd Everett Griffin](https://www.toddgriffin.me/).
