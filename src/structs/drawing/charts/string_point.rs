use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

// c:pt
use super::NumericValue;
use crate::{
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
    xml_read_loop,
};

#[derive(Clone, Default, Debug)]
pub struct StringPoint {
    numeric_value: NumericValue,
}

impl StringPoint {
    #[must_use]
    pub fn get_numeric_value(&self) -> &NumericValue {
        &self.numeric_value
    }

    pub fn get_numeric_value_mut(&mut self) -> &mut NumericValue {
        &mut self.numeric_value
    }

    pub fn set_numeric_value(&mut self, value: NumericValue) -> &mut Self {
        self.numeric_value = value;
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
                if e.name().0 == b"c:v" {
                    self.numeric_value.set_attributes(reader, e);
                }
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:pt" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:pt"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, index: u32) {
        // c:pt
        let index_str = index.to_string();
        write_start_tag(
            writer,
            "c:pt",
            vec![("idx", index_str.as_str()).into()],
            false,
        );

        // c:v
        self.numeric_value.write_to(writer);

        write_end_tag(writer, "c:pt");
    }
}
