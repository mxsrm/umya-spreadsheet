// worksheetSource
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::get_attribute,
    structs::Address,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct WorksheetSource {
    address: Address,
}

impl WorksheetSource {
    #[must_use]
    pub fn get_address(&self) -> &Address {
        &self.address
    }

    pub fn get_address_mut(&mut self) -> &mut Address {
        &mut self.address
    }

    pub fn set_address(&mut self, value: Address) -> &mut Self {
        self.address = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let mut address = Address::default();
        if let Some(v) = get_attribute(e, b"ref") {
            address.get_range_mut().set_range(v);
        }
        if let Some(v) = get_attribute(e, b"sheet") {
            address.set_sheet_name(v);
        }
        self.set_address(address);
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // worksheetSource
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let ref_str = self.address.get_range().get_range();
        attributes.push(("ref", ref_str.as_str()).into());
        if self.address.get_sheet_name() != "" {
            attributes.push(("sheet", self.address.get_sheet_name()).into());
        }
        write_start_tag(writer, "worksheetSource", attributes, true);
    }
}
