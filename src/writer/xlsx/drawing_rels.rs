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
        write_end_tag,
        write_new_line,
        write_start_tag,
    },
};
use crate::{
    helper::const_str::{
        CHART_NS,
        IMAGE_NS,
        PKG_DRAWINGS_RELS,
        REL_NS,
    },
    structs::{
        Worksheet,
        WriterManager,
    },
};

pub(crate) fn write<W: io::Seek + io::Write>(
    _worksheet: &Worksheet,
    drawing_no: &str,
    chart_no_list: &[String],
    rel_list: &[(String, String)],
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut is_write = false;

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

    // relationships
    write_start_tag(
        &mut writer,
        "Relationships",
        vec![("xmlns", REL_NS).into()],
        false,
    );

    let mut r_id = 1;
    for chart_no in chart_no_list {
        is_write = write_relationship(
            &mut writer,
            r_id,
            CHART_NS,
            format!("../charts/chart{chart_no}.xml").as_str(),
            "",
        );
        r_id += 1;
    }

    let mut r_id = 1;
    for (key, value) in rel_list {
        if key == "IMAGE" {
            is_write = write_relationship(
                &mut writer,
                r_id,
                IMAGE_NS,
                format!("../media/{value}").as_str(),
                "",
            );
        }
        r_id += 1;
    }
    write_end_tag(&mut writer, "Relationships");

    if is_write {
        let file_path = format!("{PKG_DRAWINGS_RELS}{drawing_no}.xml.rels");
        return writer_mng.add_writer(&file_path, writer);
    }
    Ok(())
}

fn write_relationship(
    writer: &mut Writer<io::Cursor<Vec<u8>>>,
    r_id: i32,
    p_type: &str,
    p_target: &str,
    p_target_mode: &str,
) -> bool {
    let r_id_str = format!("rId{r_id}");
    let mut attributes: crate::structs::AttrCollection = Vec::new();
    attributes.push(("Id", &r_id_str).into());
    attributes.push(("Type", p_type).into());
    attributes.push(("Target", p_target).into());
    if !p_target_mode.is_empty() {
        attributes.push(("TargetMode", p_target_mode).into());
    }
    write_start_tag(writer, "Relationship", attributes, true);
    true
}
