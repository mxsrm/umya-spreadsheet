// a:outerShdw
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
    PresetColor,
    RgbColorModelHex,
    SchemeColor,
};
use crate::{
    StringValue,
    reader::driver::{
        get_attribute,
        xml_read_loop,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct OuterShadow {
    blur_radius:         StringValue,
    alignment:           StringValue,
    horizontal_ratio:    StringValue,
    vertical_ratio:      StringValue,
    direction:           StringValue,
    distance:            StringValue,
    rotate_with_shape:   StringValue,
    preset_color:        Option<Box<PresetColor>>,
    scheme_color:        Option<Box<SchemeColor>>,
    rgb_color_model_hex: Option<Box<RgbColorModelHex>>,
}

impl OuterShadow {
    #[inline]
    #[must_use]
    pub fn get_blur_radius(&self) -> Option<&str> {
        self.blur_radius.get_value()
    }

    #[inline]
    pub fn set_blur_radius<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.blur_radius.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_horizontal_ratio(&self) -> Option<&str> {
        self.horizontal_ratio.get_value()
    }

    #[inline]
    pub fn set_horizontal_ratio<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.horizontal_ratio.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_vertical_ratio(&self) -> Option<&str> {
        self.vertical_ratio.get_value()
    }

    #[inline]
    pub fn set_vertical_ratio<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.vertical_ratio.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_alignment(&self) -> Option<&str> {
        self.alignment.get_value()
    }

    #[inline]
    pub fn set_alignment<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.alignment.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_direction(&self) -> Option<&str> {
        self.direction.get_value()
    }

    #[inline]
    pub fn set_direction<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.direction.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_distance(&self) -> Option<&str> {
        self.distance.get_value()
    }

    #[inline]
    pub fn set_distance<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.distance.set_value(value);
        self
    }

    #[must_use]
    pub fn get_rotate_with_shape(&self) -> Option<&str> {
        self.rotate_with_shape.get_value()
    }

    #[inline]
    pub fn set_rotate_with_shape<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.rotate_with_shape.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_preset_color(&self) -> Option<&PresetColor> {
        self.preset_color.as_deref()
    }

    #[inline]
    pub fn get_preset_color_mut(&mut self) -> Option<&mut PresetColor> {
        self.preset_color.as_deref_mut()
    }

    #[inline]
    pub fn set_preset_color(&mut self, value: PresetColor) -> &mut Self {
        self.preset_color = Some(Box::new(value));
        self
    }

    #[inline]
    #[must_use]
    pub fn get_scheme_color(&self) -> Option<&SchemeColor> {
        self.scheme_color.as_deref()
    }

    #[inline]
    pub fn get_scheme_color_mut(&mut self) -> Option<&mut SchemeColor> {
        self.scheme_color.as_deref_mut()
    }

    #[inline]
    pub fn set_scheme_color(&mut self, value: SchemeColor) -> &mut Self {
        self.scheme_color = Some(Box::new(value));
        self
    }

    #[inline]
    #[must_use]
    pub fn get_rgb_color_model_hex(&self) -> Option<&RgbColorModelHex> {
        self.rgb_color_model_hex.as_deref()
    }

    #[inline]
    pub fn get_rgb_color_model_hex_mut(&mut self) -> Option<&mut RgbColorModelHex> {
        self.rgb_color_model_hex.as_deref_mut()
    }

    #[inline]
    pub fn set_rgb_color_model_hex(&mut self, value: RgbColorModelHex) -> &mut Self {
        self.rgb_color_model_hex = Some(Box::new(value));
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        if let Some(v) = get_attribute(e, b"blurRad") {
            self.set_blur_radius(v);
        }
        if let Some(v) = get_attribute(e, b"dist") {
            self.set_distance(v);
        }
        if let Some(v) = get_attribute(e, b"dir") {
            self.set_direction(v);
        }
        if let Some(v) = get_attribute(e, b"sx") {
            self.set_horizontal_ratio(v);
        }
        if let Some(v) = get_attribute(e, b"sy") {
            self.set_vertical_ratio(v);
        }
        if let Some(v) = get_attribute(e, b"algn") {
            self.set_alignment(v);
        }
        if let Some(v) = get_attribute(e, b"rotWithShape") {
            self.set_rotate_with_shape(v);
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"a:schemeClr" => {
                        let mut obj = SchemeColor::default();
                        obj.set_attributes(reader, e, true);
                        self.set_scheme_color(obj);
                    }
                    b"a:srgbClr" => {
                        let mut obj = RgbColorModelHex::default();
                        obj.set_attributes(reader, e, true);
                        self.set_rgb_color_model_hex(obj);
                    }
                    _ => (),
                }
            },
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"a:prstClr" => {
                        let mut obj = PresetColor::default();
                        obj.set_attributes(reader, e);
                        self.set_preset_color(obj);
                    }
                    b"a:schemeClr" => {
                        let mut obj = SchemeColor::default();
                        obj.set_attributes(reader, e, false);
                        self.set_scheme_color(obj);
                    }
                    b"a:srgbClr" => {
                        let mut obj = RgbColorModelHex::default();
                        obj.set_attributes(reader, e, false);
                        self.set_rgb_color_model_hex(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:outerShdw" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:outerShdw")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:outerShdw
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if let Some(v) = self.blur_radius.get_value() {
            attributes.push(("blurRad", v).into());
        }
        if let Some(v) = self.distance.get_value() {
            attributes.push(("dist", v).into());
        }
        if let Some(v) = self.direction.get_value() {
            attributes.push(("dir", v).into());
        }
        if let Some(v) = self.horizontal_ratio.get_value() {
            attributes.push(("sx", v).into());
        }
        if let Some(v) = self.vertical_ratio.get_value() {
            attributes.push(("sy", v).into());
        }
        if let Some(v) = self.alignment.get_value() {
            attributes.push(("algn", v).into());
        }
        if let Some(v) = self.rotate_with_shape.get_value() {
            attributes.push(("rotWithShape", v).into());
        }
        write_start_tag(writer, "a:outerShdw", attributes, false);

        // a:prstClr
        if let Some(v) = &self.preset_color {
            v.write_to(writer);
        }

        // a:schemeClr
        if let Some(v) = &self.scheme_color {
            v.write_to(writer);
        }

        // a:srgbClr
        if let Some(v) = &self.rgb_color_model_hex {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:outerShdw");
    }
}
