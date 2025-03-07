use std::io;

use quick_xml::{
    Writer,
    events::{
        BytesDecl,
        Event,
    },
};

use super::{
    XlsxError,
    driver::{
        make_file_from_writer,
        write_end_tag,
        write_new_line,
        write_start_tag,
    },
};
use crate::{
    helper::const_str::{
        CONTENT_TYPES,
        CONTYPES_NS,
        PRNTR_SETTINGS_TYPE,
        REL_TYPE,
        VML_DRAWING_TYPE,
        WORKBOOK,
    },
    structs::{
        Workbook,
        WriterManager,
    },
};

pub(crate) fn write<W: io::Seek + io::Write>(
    wb: &Workbook,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let is_light = writer_mng.get_is_light();
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer
        .write_event(Event::Decl(BytesDecl::new(
            "1.0",
            Some("UTF-8"),
            Some("yes"),
        )))
        .unwrap();
    write_new_line(&mut writer);

    // Types
    write_start_tag(
        &mut writer,
        "Types",
        vec![("xmlns", CONTYPES_NS).into()],
        false,
    );

    // Default rels
    write_start_tag(
        &mut writer,
        "Default",
        vec![
            ("Extension", "rels").into(),
            ("ContentType", REL_TYPE).into(),
        ],
        true,
    );

    // Default xml
    write_start_tag(
        &mut writer,
        "Default",
        vec![
            ("Extension", "xml").into(),
            ("ContentType", "application/xml").into(),
        ],
        true,
    );

    // Default bin
    if writer_mng.has_extension("bin") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![
                ("Extension", "bin").into(),
                ("ContentType", PRNTR_SETTINGS_TYPE).into(),
            ],
            true,
        );
    }

    // Default vml
    if writer_mng.has_extension("vml") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![
                ("Extension", "vml").into(),
                ("ContentType", VML_DRAWING_TYPE).into(),
            ],
            true,
        );
    }

    // Default png
    if writer_mng.has_extension("png") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![
                ("Extension", "png").into(),
                ("ContentType", "image/png").into(),
            ],
            true,
        );
    }

    // Default jpg
    if writer_mng.has_extension("jpg") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![
                ("Extension", "jpg").into(),
                ("ContentType", "image/jpeg").into(),
            ],
            true,
        );
    }

    // Default jpeg
    if writer_mng.has_extension("jpeg") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![
                ("Extension", "jpeg").into(),
                ("ContentType", "image/jpeg").into(),
            ],
            true,
        );
    }

    // Default tiff
    if writer_mng.has_extension("tiff") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![
                ("Extension", "tiff").into(),
                ("ContentType", "image/tiff").into(),
            ],
            true,
        );
    }

    // Default emf
    if writer_mng.has_extension("emf") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![
                ("Extension", "emf").into(),
                ("ContentType", "image/x-emf").into(),
            ],
            true,
        );
    }

    // Default xlsx
    if writer_mng.has_extension("xlsx") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![
                ("Extension", "xlsx").into(),
                ("ContentType", WORKBOOK).into(),
            ],
            true,
        );
    }

    // Override
    for (part_name, content_type) in writer_mng.make_context_type_override(wb) {
        write_start_tag(
            &mut writer,
            "Override",
            vec![
                ("PartName", &part_name).into(),
                ("ContentType", &content_type).into(),
            ],
            true,
        );
    }

    write_end_tag(&mut writer, "Types");
    make_file_from_writer(
        CONTENT_TYPES,
        writer_mng.get_arv_mut(),
        writer,
        None,
        is_light,
    )?;
    Ok(())
}
