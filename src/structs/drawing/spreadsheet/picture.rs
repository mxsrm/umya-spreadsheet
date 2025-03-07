// xdr:pic
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
    BlipFill,
    NonVisualPictureProperties,
    ShapeProperties,
};
use crate::{
    reader::driver::xml_read_loop,
    structs::raw::RawRelationships,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct Picture {
    non_visual_picture_properties: NonVisualPictureProperties,
    blip_fill:                     BlipFill,
    shape_properties:              ShapeProperties,
}

impl Picture {
    #[inline]
    #[must_use]
    pub fn get_non_visual_picture_properties(&self) -> &NonVisualPictureProperties {
        &self.non_visual_picture_properties
    }

    #[inline]
    pub fn get_non_visual_picture_properties_mut(&mut self) -> &mut NonVisualPictureProperties {
        &mut self.non_visual_picture_properties
    }

    #[inline]
    pub fn set_non_visual_picture_properties(&mut self, value: NonVisualPictureProperties) {
        self.non_visual_picture_properties = value;
    }

    #[inline]
    #[must_use]
    pub fn get_blip_fill(&self) -> &BlipFill {
        &self.blip_fill
    }

    #[inline]
    pub fn get_blip_fill_mut(&mut self) -> &mut BlipFill {
        &mut self.blip_fill
    }

    #[inline]
    pub fn set_blip_fill(&mut self, value: BlipFill) {
        self.blip_fill = value;
    }

    #[inline]
    #[must_use]
    pub fn get_shape_properties(&self) -> &ShapeProperties {
        &self.shape_properties
    }

    #[inline]
    pub fn get_shape_properties_mut(&mut self) -> &mut ShapeProperties {
        &mut self.shape_properties
    }

    #[inline]
    pub fn set_shape_properties(&mut self, value: ShapeProperties) {
        self.shape_properties = value;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"xdr:nvPicPr" => {
                        self.non_visual_picture_properties.set_attributes(reader, e);
                    }
                    b"xdr:blipFill" => {
                        self.blip_fill
                            .set_attributes(reader, e, drawing_relationships);
                        }
                    b"xdr:spPr" => {
                        self.shape_properties.set_attributes(reader, e, drawing_relationships);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:pic" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:pic")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // xdr:pic
        write_start_tag(writer, "xdr:pic", vec![], false);

        // xdr:nvPicPr
        self.non_visual_picture_properties.write_to(writer);

        // xdr:blipFill
        self.blip_fill.write_to(writer, rel_list);

        // xdr:spPr
        self.shape_properties.write_to(writer, rel_list);

        write_end_tag(writer, "xdr:pic");
    }
}
