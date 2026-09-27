//! Crate-private helpers for writing sitemap XML with `quick-xml`.

use crate::ENCODING;
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
