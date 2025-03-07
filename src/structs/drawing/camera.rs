// a:camera
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
    super::EnumValue,
    PresetCameraValues,
    Rotation,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct Camera {
    preset:   EnumValue<PresetCameraValues>,
    rotation: Option<Box<Rotation>>,
}

impl Camera {
    #[inline]
    #[must_use]
    pub fn get_preset(&self) -> &PresetCameraValues {
        self.preset.get_value()
    }

    #[inline]
    pub fn set_preset(&mut self, value: PresetCameraValues) -> &mut Self {
        self.preset.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_rotation(&self) -> Option<&Rotation> {
        self.rotation.as_deref()
    }

    #[inline]
    pub fn get_rotation_mut(&mut self) -> Option<&mut Rotation> {
        self.rotation.as_deref_mut()
    }

    #[inline]
    pub fn set_rotation(&mut self, value: Rotation) -> &mut Self {
        self.rotation = Some(Box::new(value));
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        set_string_from_xml!(self, e, preset, "prst");

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:rot" {
                    let mut obj = Rotation::default();
                    obj.set_attributes(reader, e);
                    self.rotation = Some(Box::new(obj));
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:camera" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:camera")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let with_inner = self.rotation.is_some();
        // a:camera
        write_start_tag(
            writer,
            "a:camera",
            vec![("prst", self.preset.get_value_string()).into()],
            !with_inner,
        );

        if with_inner {
            // a:rot
            if let Some(v) = &self.rotation {
                v.write_to(writer);
            }
            write_end_tag(writer, "a:camera");
        }
    }
}
