// a:gd
use crate::writer::driver::*;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct ShapeGuide {
    name: String,
    fmla: String,
}
impl ShapeGuide {
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn set_name<S: Into<String>>(&mut self, value: S) {
        self.name = value.into();
    }

    #[inline]
    pub fn get_fmla(&self) -> &str {
        &self.fmla
    }

    #[inline]
    pub fn set_fmla<S: Into<String>>(&mut self, value: S) {
        self.fmla = value.into();
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(
            writer,
            "a:gd",
            vec![("name", &self.name), ("fmla", &self.fmla)],
            true,
        );
    }
}
