// a:gs
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
    RgbColorModelHex,
    SchemeColor,
};
use crate::{
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
pub struct GradientStop {
    position:            i32,
    scheme_color:        Option<Box<SchemeColor>>,
    rgb_color_model_hex: Option<Box<RgbColorModelHex>>,
}

impl GradientStop {
    #[inline]
    #[must_use]
    pub fn get_position(&self) -> i32 {
        self.position
    }

    #[inline]
    pub fn set_position(&mut self, value: i32) -> &mut GradientStop {
        self.position = value;
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
    pub fn set_scheme_color(&mut self, value: SchemeColor) -> &mut GradientStop {
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
    pub fn set_rgb_color_model_hex(&mut self, value: RgbColorModelHex) -> &mut GradientStop {
        self.rgb_color_model_hex = Some(Box::new(value));
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        if let Some(v) = get_attribute(e, b"pos") {
            self.set_position(v.parse::<i32>().unwrap());
        }

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
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
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:gs" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:gs")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:gs
        write_start_tag(
            writer,
            "a:gs",
            vec![("pos", &self.position.to_string()).into()],
            false,
        );

        // a:schemeClr
        if let Some(v) = &self.scheme_color {
            v.write_to(writer);
        }

        // a:srgbClr
        if let Some(v) = &self.rgb_color_model_hex {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:gs");
    }
}
