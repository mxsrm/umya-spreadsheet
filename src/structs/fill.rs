use std::io::Cursor;

use md5::Digest;
use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    GradientFill,
    PatternFill,
    PatternValues,
};
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Fill {
    pattern_fill:  Option<Box<PatternFill>>,
    gradient_fill: Option<Box<GradientFill>>,
}

impl Fill {
    #[inline]
    #[must_use]
    pub fn get_pattern_fill(&self) -> Option<&PatternFill> {
        self.pattern_fill.as_deref()
    }

    #[inline]
    pub fn get_pattern_fill_mut(&mut self) -> &mut PatternFill {
        if self.pattern_fill.is_some() {
            return self.pattern_fill.as_mut().unwrap();
        }
        self.set_pattern_fill(PatternFill::default());
        self.pattern_fill.as_mut().unwrap()
    }

    #[inline]
    pub fn set_pattern_fill(&mut self, value: PatternFill) -> &mut Self {
        self.pattern_fill = Some(Box::new(value));
        self.gradient_fill = None;
        self
    }

    #[inline]
    #[must_use]
    pub fn get_gradient_fill(&self) -> Option<&GradientFill> {
        self.gradient_fill.as_deref()
    }

    #[inline]
    pub fn get_gradient_fill_mut(&mut self) -> &mut GradientFill {
        if self.gradient_fill.is_some() {
            return self.gradient_fill.as_mut().unwrap();
        }
        self.set_gradient_fill(GradientFill::default());
        self.gradient_fill.as_mut().unwrap()
    }

    #[inline]
    pub fn set_gradient_fill(&mut self, value: GradientFill) -> &mut Self {
        self.pattern_fill = None;
        self.gradient_fill = Some(Box::new(value));
        self
    }

    #[inline]
    pub(crate) fn get_default_value() -> Self {
        let mut def = Self::default();
        let mut pfill = PatternFill::default();
        pfill.set_pattern_type(PatternValues::None);
        def.set_pattern_fill(pfill);
        def
    }

    #[inline]
    pub(crate) fn get_default_value_2() -> Self {
        let mut def = Self::default();
        let mut pfill = PatternFill::default();
        pfill.set_pattern_type(PatternValues::Gray125);
        def.set_pattern_fill(pfill);
        def
    }

    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}",
                match &self.pattern_fill {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "NONE".to_string()
                    }
                },
                match &self.gradient_fill {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "NONE".to_string()
                    }
                },
            ))
        )
    }

    // When opened in software such as Excel, it is visually blank.
    #[inline]
    pub(crate) fn is_visually_empty(&self) -> bool {
        !(self
            .pattern_fill
            .as_ref()
            .is_some_and(|x| !x.is_visually_empty())
            || self.gradient_fill.as_ref().is_some())
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"patternFill" {
                    let mut obj = PatternFill::default();
                    obj.set_attributes(reader, e, true);
                    self.set_pattern_fill(obj);
                }
            },
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"patternFill" => {
                        let mut obj = PatternFill::default();
                        obj.set_attributes(reader, e, false);
                        self.set_pattern_fill(obj);
                    }
                    b"gradientFill" => {
                        let mut obj = GradientFill::default();
                        obj.set_attributes(reader, e);
                        self.set_gradient_fill(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"fill" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "fill")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // fill
        write_start_tag(writer, "fill", vec![], false);

        // gradientFill
        if let Some(v) = &self.pattern_fill {
            v.write_to(writer);
        }

        // patternFill
        if let Some(v) = &self.gradient_fill {
            v.write_to(writer);
        }

        write_end_tag(writer, "fill");
    }
}
