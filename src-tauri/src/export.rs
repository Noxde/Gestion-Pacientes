use chrono::format::format;
use rusqlite::Connection;
use crate::patient::get_medical_history;
use crate::custom_types::structs::{Doc, Patient, MedicalHistory};
use crate::db::patient;
use std::collections::VecDeque;
use chrono::{Datelike, Local, Timelike};

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
const DOC_WIDTH: f32 = 595.0;
const DOC_HEIGHT: f32 = 842.0;
const X_MARGIN: f32 = 25.0;
const Y_MARGIN: f32 = 50.0;
///Font size to be used for `H1` headings.
const H1: f32 = 40.0;
///Font size to be used for `H2` headings.
const H2: f32 = 30.0;
///The font size to be used for regular test.
const FONT_SIZE: f32 = 16.0;
///Width height ratio of the font.
const FONT_WIDTH: f32 = 0.6;
///Maximum amount of characters of size `FONT_SIZE` that fit in `DOC_WIDTH` without going into the
///`X_MARGIN`.
//For this font in particular the width of the character is about 0.6 times the
//height, hence the FONT_SIZE * 0.6
const MAX_CHARS_PER_LINE: usize = ((DOC_WIDTH - (2.0 * X_MARGIN)) / (FONT_SIZE * FONT_WIDTH)) as usize;


pub fn generate_pdf(patient_id: i32, data_dir: &PathBuf, conn: &Connection, save_path: Option<String>) -> Result<Doc, String> {
    let start_time = Local::now();

    let mc = MAX_CHARS_PER_LINE;

    // Get medical history
    let his = get_medical_history(conn, patient_id, data_dir)?;

    // Create a new document.
    let mut document = Document::new();
    // Load a font.
    let font = Font::new(JETBRAINS_MONO_REGULAR.to_vec().into(), 0).unwrap();
    //Page settings
    let page_settings = PageSettings::new(DOC_WIDTH, DOC_HEIGHT);

    // PAGE ONE
    // Add a new page
    let mut page = document.start_page_with(page_settings);
    // Get the surface of the page.
    let mut surface = page.surface();
    let title = "Historia Médica";
    surface.draw_text(
        Point::from_xy(center(title, H1), Y_MARGIN + H1),
        font.clone(),
        H1,
        title,
        false,
        TextDirection::Auto,
    );

    // P1 Footer
    let full_name = format!("Paciente: {} {}", his.patient.name, his.patient.surname);
    let mut full_name_lines: Vec<String> = justify(
        full_name,
        MAX_CHARS_PER_LINE);
    //Limit the name length to 5 lines
    if full_name_lines.len() > 5 {
        full_name_lines[4].replace_range((mc - 3).., "...");
        full_name_lines.truncate(5);
    }
    //Draw the name
    for (i, line) in full_name_lines.iter().enumerate() {
        surface.draw_text(
            Point::from_xy(X_MARGIN, DOC_HEIGHT - X_MARGIN - (full_name_lines.len() - i) as f32 * 20.0),
            font.clone(),
            FONT_SIZE,
            &line,
            false,
            TextDirection::Auto,
        );
    }

    // P1 Footer
    let current_time = Local::now();
    surface.draw_text(
        Point::from_xy(X_MARGIN, DOC_HEIGHT - X_MARGIN),
        font.clone(),
        16.0,
        &format!("Generado de forma automática: {}/{}/{} {}:{:0>2}",
            current_time.day(),
            current_time.month(),
            current_time.year(),
            current_time.hour(),
            current_time.minute(),
            ),
        false,
        TextDirection::Auto,
    );
    surface.finish();
    page.finish();

    // PACIENT INFO PAGE/S
    page = document.start_page();
    surface = page.surface();

    // Draw title
    let title = "Informacion del paciente";
    surface.draw_text(
        Point::from_xy(center(title, H2), Y_MARGIN+H2),
        font.clone(),
        H2,
        title,
        false,
        TextDirection::Auto,
    );

    // Draw some more text, in a different color with an opacity and bigger font size.
    let mut patient_info_lines: Vec<String> = Vec::new();

    let mut field_lines: Vec<String> = justify(
        format!("Nombre: {}", his.patient.name),
        MAX_CHARS_PER_LINE);

    patient_info_lines.append(&mut field_lines);

    let mut field_lines: Vec<String> = justify(
        format!("Apellido: {}", his.patient.surname),
        MAX_CHARS_PER_LINE);

    patient_info_lines.append(&mut field_lines);

    let mut field_lines: Vec<String> = justify(
        format!("DNI: {}", his.patient.national_id),
        MAX_CHARS_PER_LINE);

    patient_info_lines.append(&mut field_lines);


    let mut field_lines: Vec<String> = justify(
        format!("Teléfono: {}", his.patient.phone),
        MAX_CHARS_PER_LINE);

    patient_info_lines.append(&mut field_lines);

    if let Some(medicare) = &his.patient.medicare {
        let mut field_lines: Vec<String> = justify(
            format!("Obra Social: {}", medicare),
            MAX_CHARS_PER_LINE);

        patient_info_lines.append(&mut field_lines);
    }

    if let Some(medicare_number) = &his.patient.medicare_number {
        let mut field_lines: Vec<String> = justify(
            format!("Número de Obra Social: {}", medicare_number),
            MAX_CHARS_PER_LINE);

        patient_info_lines.append(&mut field_lines);
    }

    let mut field_lines: Vec<String> = justify(
        format!("Sexo: {:?}", his.patient.sex),
        MAX_CHARS_PER_LINE);

    patient_info_lines.append(&mut field_lines);

    if let Some(gender) = &his.patient.gender {
        let mut field_lines: Vec<String> = justify(
            format!("Género: {}", gender),
            MAX_CHARS_PER_LINE);

        patient_info_lines.append(&mut field_lines);
    }

    if let Some(description) = &his.patient.description {
        let mut field_lines: Vec<String> = justify(
            format!("Descripción: {}", description),
            MAX_CHARS_PER_LINE);

        patient_info_lines.append(&mut field_lines);
    }

    let mut y = 300.0;
    for line in patient_info_lines {
        surface.draw_text(
            Point::from_xy(X_MARGIN, y),
            font.clone(),
            FONT_SIZE,
            &line,
            false,
            TextDirection::Auto,
        );
        y+=20.0;

        if y > (DOC_HEIGHT - Y_MARGIN) {

            surface.finish();
            page.finish();
            page = document.start_page();
            surface = page.surface();
        }
    }
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

    let res = save(&his.patient, data_dir, save_path, &pdf);

    let finish_time = Local::now();

    println!("Took {}ms", (finish_time - start_time).num_milliseconds());

    res
}

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

///Justifies the `text` into lines of `max_width`.
///`max_width` must be greater than one otherwise an empty String will be returned
pub fn justify(text: String, max_width: usize) -> Vec<String> {
    let mut output = Vec::new();

    if max_width < 2 {
        return output;
    }

    if text.len() <= max_width {
        output.push(text);
        return output;
    }

    let mut words: VecDeque<String> = text.split_whitespace().map(|s| s.to_string()).collect();

    //For each new line
    while !words.is_empty() {

        let mut fit: usize = 0; //How many words fit in the current line
        let mut remaining: usize = max_width; //Remaining space on the line
        let mut first: usize = 0; //Spaces to add to separate words (0 if first word)
        let mut total_w_len: usize = 0; //Sum of the length of the words

        for w in &words {
            let w_len: usize = w.len();

            //Is there space for another word
            if remaining >= (first + w_len) {
                fit+=1;
                remaining-=w_len + first;
                first = 1;
                total_w_len+=w_len;
                continue;
            }

            //If the next word is too big and there is space in the current line
            //Cut it and place a first part on the current line
            //3 = <space><char><hyphen>
            if (remaining >= (2 + first)) && (w_len > max_width) {
                //The long word is divided into a first and second part
                let mut first_part = words[fit].clone();

                //Take all space remaining minus the space (first) and the hyphen
                let second_part = first_part.split_off(remaining as usize - 1 - first);

                first_part.push('-'); //Add separator

                words[fit] = first_part; //Overwrite the word with the first part
                words.insert(fit + 1, second_part); //Insert the second part after the first

                fit+=1;
                //The piece of the cut word will take all remaining space
                total_w_len+=remaining-1; //-1 space
            }

            //There is no more space on the line
            break;
        }

        let whitespaces = max_width - total_w_len; //Length of the line - length of the words

        let spaces; //Spaces to add between words
        let mut remainder; //Number of first words that have an extra space

        if fit == words.len() {
            //If this is the last line only one space should be used (Left align)
            spaces = 1;
            remainder = 0;
        } else if fit > 1 {
            //The spaces are distributed evenly
            spaces = whitespaces / (fit - 1);
            //If the spaces is not divisible by the gaps between words (fit - 1)
            //Then the <remainder> first words will have one extra space
            remainder = whitespaces % (fit -1);
            //Avoid dividing by zero, if fit is one spaces do not matter
        } else {
            //If there is only one word (fit == 1) there will no spaces
            spaces = 0;
            remainder = 0;
        }

        //The line to be added to the output
        let mut new_line = String::new();
        //Insert the first word (does not require an space)
        if fit > 0 {
            new_line.push_str(&words.pop_front().expect("words should not be empty"));
            fit-=1;
        }
        //For each word to insert in the line
        for _ in 0..fit {
            //Spaces to add before inserting the next word
            //Is equal to spaces, plus one if the word is one of the <remainder> first
            let current_spaces = if remainder > 0 {
                remainder-=1;
                spaces + 1
            } else {
                spaces
            };

            new_line.push_str(&" ".repeat(current_spaces)); //Insert the spaces
            new_line.push_str(&words.pop_front().expect("words should not be empty")); //Insert the word
        }

        //Insert the newline
        output.push(new_line);
    }

    output
}

///Returns the necessary x offset to draw the text in the center of the page
fn center(text: &str, font_size: f32) -> f32 {
    let line_width = text.len() as f32 * font_size * FONT_WIDTH;
    ((DOC_WIDTH - line_width) / 2.0).max(25.0)
}
