use crate::model::{FilePath, TestSuite};
use std::fs;
use quick_xml::de::from_str;

pub fn file_to_report(path: &FilePath) -> Option<TestSuite> {
    let content = fs::read_to_string(&path.path)
        .map_err(|_| { eprintln!("Can't read content of file {}", path.path); })
        .ok()?;
    from_str::<TestSuite>(&content)
        .map_err(|_| { eprintln!("Can't parse file {}", path.path) })
        .ok()
}