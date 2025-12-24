use chrono::format::format;
use rusqlite::Connection;
use crate::patient::get_medical_history;
use crate::custom_types::structs::{Doc, Patient, MedicalHistory};
use crate::db::patient;

use std::num::NonZeroU64;
use std::sync::Arc;
use std::{fs, path, path::PathBuf};

use krilla::color::rgb;
use krilla::geom::{PathBuilder, Point, Rect, Size};
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;
use krilla::paint::{Fill, FillRule, LinearGradient, SpreadMethod, Stop};
use krilla::text::{
    Font,
    TextDirection
};
use krilla::configure::ValidationError;
use krilla::embed::{AssociationKind, EmbedError, EmbeddedFile, MimeType};
use krilla::error::KrillaError;
use krilla::metadata::{DateTime, Metadata};
use krilla::tagging::TagTree;

use krilla::annotation::{LinkAnnotation, Target};
use krilla::action::{Action, LinkAction};
use krilla::image::Image;


use krilla::{Data, Document};
use krilla::geom::{ Transform};

const JETBRAINS_MONO_REGULAR: &[u8] = include_bytes!("../assets/fonts/JetBrainsMonoNLNerdFontMono-Regular.ttf");

/// Save the export file in a custom folder or with a custom name, if this fails or `save_path` is
/// None, retries to save it in `data_dir/exports` with a generated name.
pub fn save(patient: &Patient, data_dir: &PathBuf, save_path: Option<String>, pdf: &Vec<u8>) -> Result<Doc, String> {
    let gen_file_name = || -> String {
        format!("{}_{}_historia_medica.pdf",
            patient.name,
            patient.surname)
            .replace(" ", "_").to_lowercase()
    };

    let gen_path_and_name = || -> (String, PathBuf) {
        let name = gen_file_name();
        let path = data_dir.join("exports").join(&name);
        (name, path)
    };

    //If exporting to a custom path fails, try again in app_dir
    let mut retry = true;
    // --- RESOLVE THE FINAL PATH ---
    let (mut file_name, mut file_path) = match save_path {
        Some(path_str) => {
            let mut p = PathBuf::from(path_str);

            if p.is_dir() {
                // Folder → append generated filename
                let name = gen_file_name();
                p = p.join(&name);
                (name, p)
            } else {
                // File → use the path directly
                match p.file_name() {
                    Some(name) => (name.to_string_lossy().to_string(), p),
                    None => gen_path_and_name(),
                }
            }
        }
        None => {
            retry = false;
            gen_path_and_name()
        }
    };

    // Write the PDF to a file.
    let result = std::fs::write(&file_path, &pdf);

    // If the custom path failed, try to save in data_dir
    if let Err(save_err) = result {
        if retry {
            (file_name, file_path) = gen_path_and_name();
            std::fs::write(&file_path, &pdf)
                .map_err(|e| format!("Cannot write PDF file to {}: {}", file_path.display(), e))?;
        } else {
            return Err(format!("Cannot write PDF file to {}: {}", file_path.display(), save_err));

        }
    }

    println!("Saved PDF to '{}'", file_path.display());

    Ok(Doc {
        name: file_name,
        path: file_path
            .to_string_lossy()
            .to_string(),
    })
}

pub fn generate_pdf(patient_id: i32, data_dir: &PathBuf, conn: &Connection, save_path: Option<String>) -> Result<Doc, String> {
    // Get medical history
    let his = get_medical_history(conn, patient_id, data_dir)?;

    println!("{:?}", his);

    // Create a new document.
    let mut document = Document::new();
    // Load a font.
    let font = Font::new(JETBRAINS_MONO_REGULAR.to_vec().into(), 0).unwrap();
    // Add a new page
    let mut page = document.start_page_with(PageSettings::new(1000.0, 2000.0));
    // Get the surface of the page.
    let mut surface = page.surface();
    // Draw some text.
    surface.draw_text(
        Point::from_xy(0.0, 25.0),
        font.clone(),
        40.0,
        "Historia Medica",
        false,
        TextDirection::Auto,
    );

    surface.set_fill(Some(Fill {
        paint: rgb::Color::new(255, 0, 0).into(),
        opacity: NormalizedF32::new(0.5).unwrap(),
        rule: Default::default(),
    }));
    // Draw some more text, in a different color with an opacity and bigger font size.
    surface.draw_text(
        Point::from_xy(0.0, 50.0),
        font.clone(),
        16.0,
        "This text has font size 16!",
        false,
        TextDirection::Auto,
    );
    surface.finish();
    page.finish();

    let mut page = document.start_page();
    let mut surface = page.surface();


    let data = std::fs::read("src/tests/test_data/test_embed4.jpg").unwrap();
    let image = Image::from_jpeg(data.into(), false).unwrap();
    let size = image.size();
    surface.draw_image(image, Size::from_wh(size.0 as f32, size.1 as f32).unwrap());


    let data = std::fs::read("src/tests/test_data/test_embed5.png").unwrap();
    let image = Image::from_png(data.into(), false).unwrap();
    let size = image.size();
    surface.draw_image(image, Size::from_wh(size.0 as f32, size.1 as f32).unwrap());

    // Draw some text.
    surface.draw_text(
        Point::from_xy(0.0, 25.0),
        font.clone(),
        14.0,
        "This text has font size 14!",
        false,
        TextDirection::Auto,
    );

    // Finish the page.
    surface.finish();

    // Add annotation
    page.add_annotation(
        LinkAnnotation::new(
            Rect::from_xywh(50.0, 50.0, 100.0, 100.0).unwrap(),
            Target::Action(LinkAction::new("https://www.youtube.com".to_string()).into()),
        )
        .into(),
    );
    page.finish();

    let data = std::fs::read("src/tests/test_data/test_embed1.pdf").unwrap();
    let embed_file = EmbeddedFile {
        path: "test_embed1.pdf".to_string(),
        mime_type: Some(MimeType::new("application/pdf").unwrap()),
        description: Some("The description of the file.".to_string()),
        association_kind: AssociationKind::Supplement,
        data: data.into(),
        modification_date: Some(DateTime::new(2001)),
        compress: Some(false),
        location: None,
    };

    document.embed_file(embed_file).unwrap();

    let data = std::fs::read("src/tests/test_data/test_embed2.docx").unwrap();
    let embed_file = EmbeddedFile {
        path: "test_embed2.docx".to_string(),
        mime_type: Some(MimeType::new("application/vnd.openxmlformats-officedocument.wordprocessingml.document").unwrap()),
        description: Some("The description of the file.".to_string()),
        association_kind: AssociationKind::Supplement,
        data: data.into(),
        modification_date: Some(DateTime::new(2001)),
        compress: Some(false),
        location: None,
    };

    document.embed_file(embed_file).unwrap();

    let data = std::fs::read("src/tests/test_data/test_embed3.mp4").unwrap();
    let embed_file = EmbeddedFile {
        path: "test_embed3.mp4".to_string(),
        mime_type: Some(MimeType::new("video/mp4").unwrap()),
        description: Some("The description of the file.".to_string()),
        association_kind: AssociationKind::Supplement,
        data: data.into(),
        modification_date: Some(DateTime::new(2001)),
        compress: Some(false),
        location: None,
    };

    document.embed_file(embed_file).unwrap();

    let data = std::fs::read("src/tests/test_data/test_embed4.jpg").unwrap();
    let embed_file = EmbeddedFile {
        path: "test_embed4.jpg".to_string(),
        mime_type: Some(MimeType::new("image/jpg").unwrap()),
        description: Some("The description of the file.".to_string()),
        association_kind: AssociationKind::Unspecified,
        data: data.into(),
        modification_date: Some(DateTime::new(2001)),
        compress: Some(false),
        location: None,
    };

    document.embed_file(embed_file).unwrap();

    let data = std::fs::read("src/tests/test_data/test_embed5.png").unwrap();
    let embed_file = EmbeddedFile {
        path: "test_embed5.png".to_string(),
        mime_type: Some(MimeType::new("image/png").unwrap()),
        description: Some("The description of the file.".to_string()),
        association_kind: AssociationKind::Unspecified,
        data: data.into(),
        modification_date: Some(DateTime::new(2001)),
        compress: Some(false),
        location: None,
    };

    document.embed_file(embed_file).unwrap();

    let pdf = document.finish().unwrap();

    save(&his.patient, data_dir, save_path, &pdf)
}
