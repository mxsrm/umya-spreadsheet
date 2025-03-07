// xdr:nvSpPr
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::NonVisualDrawingProperties;
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct NonVisualShapeProperties {
    non_visual_drawing_properties: NonVisualDrawingProperties,
}

impl NonVisualShapeProperties {
    #[inline]
    #[must_use]
    pub fn get_non_visual_drawing_properties(&self) -> &NonVisualDrawingProperties {
        &self.non_visual_drawing_properties
    }

    #[inline]
    pub fn get_non_visual_drawing_properties_mut(&mut self) -> &mut NonVisualDrawingProperties {
        &mut self.non_visual_drawing_properties
    }

    #[inline]
    pub fn set_non_visual_drawing_properties(&mut self, value: NonVisualDrawingProperties) {
        self.non_visual_drawing_properties = value;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"xdr:cNvPr" {
                    self.non_visual_drawing_properties
                        .set_attributes(reader, e, true);
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"xdr:cNvPr" {
                    self.non_visual_drawing_properties
                        .set_attributes(reader, e, false);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:nvSpPr" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:nvSpPr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, ole_id: usize) {
        // xdr:nvSpPr
        write_start_tag(writer, "xdr:nvSpPr", vec![], false);

        // xdr:cNvPr
        self.non_visual_drawing_properties.write_to(writer, ole_id);

        // xdr:cNvSpPr
        write_start_tag(writer, "xdr:cNvSpPr", vec![], true);

        write_end_tag(writer, "xdr:nvSpPr");
    }
}
