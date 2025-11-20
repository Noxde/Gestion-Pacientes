use crate::custom_types::enums::FileType;
use std::str::FromStr;

#[test]
fn test_file_type() {
    let ft = FileType::PDF;
    assert_eq!(ft.to_mime(), "application/pdf");

    let ft = FileType::PNG;
    assert_eq!(ft.to_mime(), "image/png");

    let ft = FileType::DOCX;
    assert_eq!(ft.to_mime(), "application/vnd.openxmlformats-officedocument.wordprocessingml.document");

    FileType::from_str("this/should/work.docx").unwrap();

    FileType::from_str("/and/this/one.jpeg").unwrap();

    FileType::from_str("this_one_too.jpg").unwrap();

    let res = FileType::from_str("invalid.extension");
    assert_eq!(res.err().unwrap(), "Unsupported file type: invalid.extension");

    let res = FileType::from_str("/this/is/a/folder/");
    assert_eq!(res.err().unwrap(), "Unsupported file type: /this/is/a/folder/");
}
