use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::get_attribute,
    structs::DoubleValue,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct PageMargins {
    left:   DoubleValue,
    right:  DoubleValue,
    top:    DoubleValue,
    bottom: DoubleValue,
    header: DoubleValue,
    footer: DoubleValue,
}
impl PageMargins {
    #[inline]
    #[must_use]
    pub fn get_left(&self) -> f64 {
        self.left.get_value()
    }

    #[inline]
    pub fn set_left(&mut self, value: f64) -> &mut Self {
        self.left.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_right(&self) -> f64 {
        self.right.get_value()
    }

    #[inline]
    pub fn set_right(&mut self, value: f64) -> &mut Self {
        self.right.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_top(&self) -> f64 {
        self.top.get_value()
    }

    #[inline]
    pub fn set_top(&mut self, value: f64) -> &mut Self {
        self.top.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_bottom(&self) -> f64 {
        self.bottom.get_value()
    }

    #[inline]
    pub fn set_bottom(&mut self, value: f64) -> &mut Self {
        self.bottom.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_header(&self) -> f64 {
        self.header.get_value()
    }

    #[inline]
    pub fn set_header(&mut self, value: f64) -> &mut Self {
        self.header.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_footer(&self) -> f64 {
        self.footer.get_value()
    }

    #[inline]
    pub fn set_footer(&mut self, value: f64) -> &mut Self {
        self.footer.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.left
            .set_value_string(get_attribute(e, b"left").unwrap());
        self.right
            .set_value_string(get_attribute(e, b"right").unwrap());
        self.top.set_value_string(get_attribute(e, b"top").unwrap());
        self.bottom
            .set_value_string(get_attribute(e, b"bottom").unwrap());
        self.header
            .set_value_string(get_attribute(e, b"header").unwrap());
        self.footer
            .set_value_string(get_attribute(e, b"footer").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // pageMargins
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let left = self.left.get_value_string();
        attributes.push(("left", &left).into());
        let right = self.right.get_value_string();
        attributes.push(("right", &right).into());
        let top = self.top.get_value_string();
        attributes.push(("top", &top).into());
        let bottom = self.bottom.get_value_string();
        attributes.push(("bottom", &bottom).into());
        let header = self.header.get_value_string();
        attributes.push(("header", &header).into());
        let footer = self.footer.get_value_string();
        attributes.push(("footer", &footer).into());
        write_start_tag(writer, "pageMargins", attributes, true);
    }
}
