// a:alpha
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct Alpha {
    val: Box<str>,
}
impl Alpha {
    #[inline]
    #[must_use]
    pub fn get_val(&self) -> &str {
        &self.val
    }

    #[inline]
    pub fn set_val<S: Into<String>>(&mut self, value: S) {
        self.val = value.into().into_boxed_str();
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.set_val(get_attribute(e, b"val").unwrap());
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:alpha
        write_start_tag(writer, "a:alpha", vec![("val", &self.val).into()], true);
    }
}
