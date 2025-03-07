// stop
use std::io::Cursor;

use md5::Digest;
use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    Color,
    DoubleValue,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct GradientStop {
    position: DoubleValue,
    color:    Color,
}

impl GradientStop {
    #[inline]
    #[must_use]
    pub fn get_position(&self) -> f64 {
        self.position.get_value()
    }

    #[inline]
    pub fn set_position(&mut self, value: f64) -> &mut Self {
        self.position.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_color(&self) -> &Color {
        &self.color
    }

    #[inline]
    pub fn get_color_mut(&mut self) -> &mut Color {
        &mut self.color
    }

    #[inline]
    pub fn set_color(&mut self, value: Color) -> &mut Self {
        self.color = value;
        self
    }

    #[inline]
    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}",
                &self.position.get_value_string(),
                &self.color.get_hash_code(),
            ))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, position, "position");

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"color" {
                    let mut obj = Color::default();
                    obj.set_attributes(reader, e, true);
                    self.set_color(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"stop" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "stop")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // stop
        write_start_tag(
            writer,
            "stop",
            vec![("position", &self.position.get_value_string()).into()],
            false,
        );

        // color
        self.color.write_to_color(writer);

        write_end_tag(writer, "stop");
    }
}
