// colFields
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::{
    reader::driver::xml_read_loop,
    structs::Field,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct ColumnFields {
    list: Vec<Field>,
}
impl ColumnFields {
    #[inline]
    #[must_use]
    pub fn get_list(&self) -> &[Field] {
        &self.list
    }

    #[inline]
    pub fn get_list_mut(&mut self) -> &mut Vec<Field> {
        &mut self.list
    }

    #[inline]
    pub fn add_list_mut(&mut self, value: Field) -> &mut Self {
        self.list.push(value);
        self
    }

    #[inline]
    #[allow(unused_variables)]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"field" {
                    let mut obj = Field::default();
                    obj.set_attributes(reader, e);
                    self.add_list_mut(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"colFields" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "colFields")
        );
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // colFields
        write_start_tag(
            writer,
            "colFields",
            vec![("count", self.list.len().to_string()).into()],
            false,
        );

        // i
        for i in &self.list {
            i.write_to(writer);
        }

        write_end_tag(writer, "colFields");
    }
}
