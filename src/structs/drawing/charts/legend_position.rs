// c:legendPos
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    super::super::EnumValue,
    LegendPositionValues,
};
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct LegendPosition {
    val: EnumValue<LegendPositionValues>,
}
impl LegendPosition {
    #[must_use]
    pub fn get_val(&self) -> &LegendPositionValues {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: LegendPositionValues) -> &mut LegendPosition {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:legendPos
        write_start_tag(
            writer,
            "c:legendPos",
            vec![("val", self.val.get_value_string()).into()],
            true,
        );
    }
}
