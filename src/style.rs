use quick_xml::events::*;
use quick_xml::Writer;
use std::io::{Seek, Write};
use zip::ZipWriter;

use errors::Result;
use xml::Xml;

#[derive(Debug, Default)]
pub struct Style<'a> {
  pub name: &'a str,
  p_pr: Vec<Event<'a>>,
  r_pr: Vec<Event<'a>>,
}

macro_rules! push_empty_event {
  ($vec:expr, $tag:tt, $val:expr) => {{
    let mut bytes_start = BytesStart::borrowed($tag, $tag.len());
    bytes_start.push_attribute(("w:val", $val));
    $vec.push(Event::Empty(bytes_start));
  }};
}

impl<'a> Style<'a> {
  pub fn with_name(mut self, name: &'a str) -> Self {
    self.name = name;
    self
  }

  pub fn with_jc(mut self, justification: &Justification) -> Self {
    push_empty_event!(self.p_pr, b"w:jc", justification.as_str());
    self
  }

  pub fn with_sz(mut self, size: usize) -> Self {
    push_empty_event!(self.r_pr, b"w:sz", size.to_string().as_str());
    self
  }

  pub fn with_color(mut self, color: &'a str) -> Self {
    push_empty_event!(self.r_pr, b"w:color", color);
    self
  }
}

impl<'a> Xml<'a> for Style<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_events!(writer, (b"w:style", "w:type", "paragraph", "w:styleId", self.name) {
      (b"w:name", "w:val", self.name)
      b"w:pPr" {[
        for event in &self.p_pr {
          writer.write_event(event)?;
        }
      ]}
      b"w:rPr" {[
        for event in &self.r_pr {
          writer.write_event(event)?;
        }
      ]}
    });
    Ok(())
  }
}

#[derive(Debug)]
pub enum Justification {
  Start,
  End,
  Center,
  Both,
  Distribute,
}

impl Justification {
  pub fn as_str(&self) -> &str {
    match *self {
      Justification::Start => "start",
      Justification::End => "end",
      Justification::Center => "center",
      Justification::Both => "both",
      Justification::Distribute => "distribute",
    }
  }
}
