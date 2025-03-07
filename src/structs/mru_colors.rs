// mruColors
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::Color;
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub(crate) struct MruColors {
    color: Vec<Color>,
}

impl MruColors {
    #[inline]
    pub(crate) fn get_color(&self) -> &[Color] {
        &self.color
    }

    #[inline]
    pub(crate) fn get_color_mut(&mut self) -> &mut Vec<Color> {
        &mut self.color
    }

    #[inline]
    pub(crate) fn set_color(&mut self, value: Color) -> &mut Self {
        self.color.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"color" {
                    let mut obj = Color::default();
                    obj.set_attributes(reader, e, true);
                    self.set_color(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"mruColors" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "mruColors")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if !self.color.is_empty() {
            // mruColors
            write_start_tag(writer, "mruColors", vec![], false);

            // color
            for color in &self.color {
                color.write_to_color(writer);
            }

            write_end_tag(writer, "mruColors");
        }
    }
}
