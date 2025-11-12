use rusqlite::Connection;
use crate::patient::get_all_data;
use crate::custom_types::structs::{Doc, Patient};
use krilla::Document;
use std::{fs, path::Path};

use std::path;
use std::path::PathBuf;

use krilla::color::rgb;
use krilla::geom::{PathBuilder, Point};
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;
use krilla::paint::Fill;
use krilla::paint::{FillRule, LinearGradient, SpreadMethod, Stop};
use krilla::text::Font;
use krilla::text::TextDirection;
use krilla::configure::ValidationError;
use krilla::embed::{AssociationKind, EmbedError, EmbeddedFile, MimeType};
use krilla::error::KrillaError;
use krilla::metadata::{DateTime, Metadata};
use krilla::tagging::TagTree;

use krilla::annotation::{LinkAnnotation, Target};
use krilla::geom::Rect;
use krilla::action::{Action, LinkAction};

const JETBRAINS_MONO_REGULAR: &[u8] = include_bytes!("../assets/fonts/JetBrainsMonoNLNerdFontMono-Regular.ttf");

pub fn generate_pdf(conn: &Connection, patient_id: i32) -> Result<Doc, String> {
    // Create a new document.
    let mut document = Document::new();
    // Load a font.
    let font = Font::new(JETBRAINS_MONO_REGULAR.to_vec().into(), 0).unwrap();
    // Add a new page with dimensions 200x200.
    let mut page = document.start_page_with(PageSettings::new(200.0, 200.0));
    // Get the surface of the page.
    let mut surface = page.surface();
    // Draw some text.
    surface.draw_text(
        Point::from_xy(0.0, 25.0),
        font.clone(),
        14.0,
        "This text has font size 14!",
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

    println!("{:?}",document.embed_file(embed_file));

    let data = std::fs::read("src/tests/test_data/test_embed2.xlsx").unwrap();
    let embed_file = EmbeddedFile {
        path: "test_embed2.xlsx".to_string(),
        mime_type: Some(MimeType::new("application/vnd.ms-excel").unwrap()),
        description: Some("The description of the file.".to_string()),
        association_kind: AssociationKind::Supplement,
        data: data.into(),
        modification_date: Some(DateTime::new(2001)),
        compress: Some(false),
        location: None,
    };

    println!("{:?}",document.embed_file(embed_file));

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

    println!("{:?}",document.embed_file(embed_file));

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

    println!("{:?}",document.embed_file(embed_file));

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

    println!("{:?}",document.embed_file(embed_file));


    let pdf = document.finish().unwrap();
    let path = path::absolute("basic.pdf").unwrap();
    let path_str = path.to_string_lossy().to_string();
    eprintln!("Saved PDF to '{}'", path.display());

    // Write the PDF to a file.
    std::fs::write(path, &pdf).unwrap();

    Ok(Doc {
        name: String::from("basic.pdf"),
        path: format!("{}",path_str),
    })
}
