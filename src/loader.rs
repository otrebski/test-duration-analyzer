use std::collections::HashSet;
use std::fs;
use crate::model::FilePath;

pub fn list_xml_files_in_dir(path: &String) -> Vec<FilePath> {
    let mut xml_files = Vec::new();
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let is_xml = path.extension().and_then(|ext| ext.to_str()).unwrap_or("").eq_ignore_ascii_case("xml");
                        let starts_with_test = path.file_name().and_then(|name| name.to_str()).unwrap_or("").starts_with("TEST");
                        if is_xml && starts_with_test {
                            xml_files.push(FilePath { path: path.to_str().unwrap().to_string() })
                        }
                    }
                    Err(_e) => eprintln!("Can't read file in {}", path)
                }
            }
        }
        Err(_e) => eprintln!("Can't list files in directory {}", path)
    }
    xml_files
}

pub fn list_xml_files_in_dirs(paths: Vec<String>) -> Vec<FilePath> {
    let unique: Vec<String> = paths.into_iter().collect::<HashSet<_>>().into_iter().collect();
    unique
        .iter()
        .map(|path| list_xml_files_in_dir(path))
        .flatten()
        .collect()
}