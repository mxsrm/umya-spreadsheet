// dataValidations
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::DataValidation;
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Default, Debug, Clone)]
pub struct DataValidations {
    data_validation_list: Vec<DataValidation>,
}

impl DataValidations {
    #[inline]
    #[must_use]
    pub fn get_data_validation_list(&self) -> &[DataValidation] {
        &self.data_validation_list
    }

    #[inline]
    pub fn get_data_validation_list_mut(&mut self) -> &mut Vec<DataValidation> {
        &mut self.data_validation_list
    }

    #[inline]
    pub fn set_data_validation_list(&mut self, value: impl Into<Vec<DataValidation>>) -> &mut Self {
        self.data_validation_list = value.into();
        self
    }

    #[inline]
    pub fn add_data_validation_list(&mut self, value: DataValidation) -> &mut Self {
        self.data_validation_list.push(value);
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
                if e.name().into_inner() == b"dataValidation" {
                    let mut obj = DataValidation::default();
                    obj.set_attributes(reader, e, true);
                    self.add_data_validation_list(obj);
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"dataValidation" {
                    let mut obj = DataValidation::default();
                    obj.set_attributes(reader, e, false);
                    self.add_data_validation_list(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"dataValidations" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "dataValidations")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // dataValidations
        let mut attributes: crate::structs::AttrCollection = Vec::new();

        let count = self.data_validation_list.len().to_string();
        attributes.push(("count", &count).into());

        write_start_tag(writer, "dataValidations", attributes, false);

        for obj in &self.data_validation_list {
            obj.write_to(writer);
        }

        write_end_tag(writer, "dataValidations");
    }
}
