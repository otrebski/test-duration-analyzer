use crate::model::{FilePath, TestSuite};
use std::fs;
use quick_xml::de::from_str;

pub fn file_to_report(path: &FilePath) -> Option<TestSuite> {
    let result = match fs::read_to_string(&path.path) {
        Ok(content) => match from_str::<TestSuite>(&content) {
            Ok(test_suite) => Some(test_suite),
            Err(_e) => {
                eprintln!("Can't parse file {}", &path.path);
                None
            }
        },
        Err(_e) => {
            eprintln!("Can't read content of {}", &path.path);
            None
        }
    };
    result
}