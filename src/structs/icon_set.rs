use std::io::Cursor;

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
    ConditionalFormatValueObject,
};
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct IconSet {
    cfvo_collection:  Vec<ConditionalFormatValueObject>,
    color_collection: Vec<Color>,
}

impl IconSet {
    #[inline]
    #[must_use]
    pub fn get_cfvo_collection(&self) -> &[ConditionalFormatValueObject] {
        &self.cfvo_collection
    }

    #[inline]
    pub fn set_cfvo_collection(
        &mut self,
        value: impl Into<Vec<ConditionalFormatValueObject>>,
    ) -> &mut Self {
        self.cfvo_collection = value.into();
        self
    }

    #[inline]
    pub fn add_cfvo_collection(&mut self, value: ConditionalFormatValueObject) -> &mut Self {
        self.cfvo_collection.push(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_color_collection(&self) -> &[Color] {
        &self.color_collection
    }

    #[inline]
    pub fn set_color_collection(&mut self, value: impl Into<Vec<Color>>) -> &mut Self {
        self.color_collection = value.into();
        self
    }

    #[inline]
    pub fn add_color_collection(&mut self, value: Color) -> &mut Self {
        self.color_collection.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
                Event::Empty(ref e) => {
                    match e.name().into_inner() {
                        b"cfvo" => {
                            let mut obj = ConditionalFormatValueObject::default();
                            obj.set_attributes(reader, e, true);
                            self.cfvo_collection.push(obj);
                        }
                        b"color" => {
                            let mut obj = Color::default();
                            obj.set_attributes(reader, e, true);
                            self.color_collection.push(obj);
                        }
                        _ => (),
                    }
                },
                Event::End(ref e) => {
                    if e.name().into_inner() == b"dataBar" {
                        return
                    }
                },
                Event::Eof => panic!("Error: Could not find {} end element", "dataBar")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // dataBar
        write_start_tag(writer, "dataBar", vec![], false);

        // cfvo
        for v in &self.cfvo_collection {
            v.write_to(writer);
        }

        // color
        for v in &self.color_collection {
            v.write_to_color(writer);
        }

        write_end_tag(writer, "dataBar");
    }
}
