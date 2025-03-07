// a:lin
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::super::super::{
    BooleanValue,
    Int32Value,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct LinearGradientFill {
    angle:  Int32Value,
    scaled: BooleanValue,
}

impl LinearGradientFill {
    #[inline]
    #[must_use]
    pub fn get_angle(&self) -> i32 {
        self.angle.get_value()
    }

    #[inline]
    pub fn set_angle(&mut self, value: i32) -> &mut LinearGradientFill {
        self.angle.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_scaled(&self) -> bool {
        self.scaled.get_value()
    }

    #[inline]
    pub fn set_scaled(&mut self, value: bool) -> &mut LinearGradientFill {
        self.scaled.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, angle, "ang");
        set_string_from_xml!(self, e, scaled, "scaled");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lin
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let ang = self.angle.get_value_string();
        if self.angle.has_value() {
            attributes.push(("ang", &ang).into());
        }
        if self.scaled.has_value() {
            attributes.push(("scaled", self.scaled.get_value_string()).into());
        }
        write_start_tag(writer, "a:lin", attributes, true);
    }
}
