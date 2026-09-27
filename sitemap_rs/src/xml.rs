//! Crate-private helpers for writing sitemap XML with `quick-xml`.

use crate::ENCODING;
use jiff::Zoned;
use jiff::civil::DateTime;
use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesText, Event};
use std::io::{self, Write};

/// Writes a complete XML document: the XML declaration, the root element written by `write_root`,
/// and a trailing newline.
///
/// Elements are indented with tabs and empty elements are written as `<tag />`.
pub fn write_document<W: Write>(
    writer: W,
    write_root: impl FnOnce(&mut Writer<W>) -> io::Result<()>,
) -> io::Result<()> {
    let mut writer: Writer<W> = Writer::new_with_indent(writer, b'\t', 1);
    writer.config_mut().add_space_before_slash_in_empty_elements = true;

    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some(ENCODING), None)))?;
    write_root(&mut writer)?;
    writer.get_mut().write_all(b"\n")
}

/// Writes `<name>text</name>`, escaping `text`.
pub fn write_text_element<W: Write>(
    writer: &mut Writer<W>,
    name: &str,
    text: &str,
) -> io::Result<()> {
    writer
        .create_element(name)
        .write_text_content(BytesText::new(text))?;
    Ok(())
}

/// Formats `date` as a W3C Datetime with seconds precision and a `±hh:mm` offset, e.g.
/// `1998-01-15T04:20:00+00:00`.
///
/// Fractional seconds are truncated, UTC is written as `+00:00` rather than `Z`, and offsets are
/// rounded to the nearest minute.
///
/// The digits are written by hand because both `jiff`'s printers and `write!` are measurably slower
/// when writing 50,000 dates.
pub fn format_date(date: &Zoned) -> String {
    let datetime: DateTime = date.datetime();
    let offset_seconds: i32 = date.offset().seconds();
    let offset_minutes: u32 = (offset_seconds.unsigned_abs() + 30) / 60;

    // jiff limits years to `-9999..=9999` and offsets to under 26 hours
    let year: u16 = datetime.year().unsigned_abs();
    let century: u8 = u8::try_from(year / 100).expect("jiff years have at most 4 digits");
    let offset_hours: u8 =
        u8::try_from(offset_minutes / 60).expect("jiff offsets are under 26 hours");

    // `YYYY-MM-DDThh:mm:ss+hh:mm` is 25 bytes
    let mut formatted: String = String::with_capacity(25);
    if datetime.year() < 0 {
        formatted.push('-');
    }
    push_two_digits(&mut formatted, century);
    push_two_digits(&mut formatted, u8::try_from(year % 100).expect("below 100"));
    formatted.push('-');
    push_two_digits(&mut formatted, datetime.month().unsigned_abs());
    formatted.push('-');
    push_two_digits(&mut formatted, datetime.day().unsigned_abs());
    formatted.push('T');
    push_two_digits(&mut formatted, datetime.hour().unsigned_abs());
    formatted.push(':');
    push_two_digits(&mut formatted, datetime.minute().unsigned_abs());
    formatted.push(':');
    push_two_digits(&mut formatted, datetime.second().unsigned_abs());
    formatted.push(if offset_seconds < 0 { '-' } else { '+' });
    push_two_digits(&mut formatted, offset_hours);
    formatted.push(':');
    push_two_digits(
        &mut formatted,
        u8::try_from(offset_minutes % 60).expect("below 60"),
    );
    formatted
}

/// Pushes `value` (`0..=99`) as two zero-padded digits.
fn push_two_digits(formatted: &mut String, value: u8) {
    formatted.push(char::from(b'0' + value / 10));
    formatted.push(char::from(b'0' + value % 10));
}
