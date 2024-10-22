use std::collections::HashSet;
use std::fs;
use crate::model::FilePath;

pub fn list_xml_files_in_dir(path: &String) -> Vec<FilePath> {
    fs::read_dir(path)
        .map(|entries| {
            entries.filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                let is_xml = path.extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("")
                    .eq_ignore_ascii_case("xml");
                let starts_with_test = path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("")
                    .starts_with("TEST");
                if is_xml && starts_with_test {
                    Some(FilePath { path: path.to_str()?.to_string() })
                } else {
                    None
                }
            }).collect()
        })
        .unwrap_or_else(|_| {
            eprintln!("Can't list files in directory {}", path);
            Vec::new()
        })
}

pub fn list_xml_files_in_dirs(paths: Vec<String>) -> Vec<FilePath> {
    let unique: Vec<String> = paths.into_iter().collect::<HashSet<_>>().into_iter().collect();
    unique
        .iter()
        .map(|path| list_xml_files_in_dir(path))
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_list_xml_files_in_dir() {
        //given
        let dir = tempdir().unwrap();
        File::create(&dir.path().join("TEST-a.xml")).unwrap();
        File::create(&dir.path().join("TEST-b.xml")).unwrap();

        //when
        let dirs = list_xml_files_in_dir(&dir.path().to_string_lossy().to_string());

        //tgeb
        assert_eq!(dirs.len(), 2)
    }

    #[test]
    fn test_list_xml_files_in_dir_ignore_no_test() {
        //given
        let dir = tempdir().unwrap();
        File::create(&dir.path().join("a.xml")).unwrap();
        File::create(&dir.path().join("TEST-b.xml")).unwrap();

        //when
        let dirs = list_xml_files_in_dir(&dir.path().to_string_lossy().to_string());

        //then
        assert_eq!(dirs.len(), 1)
    }

    #[test]
    fn test_list_xml_files_in_dir_ignore_non_xml() {
        //given
        let dir = tempdir().unwrap();
        File::create(&dir.path().join("TEST-a.xmx")).unwrap();
        File::create(&dir.path().join("TEST-b.xml")).unwrap();

        //when
        let dirs = list_xml_files_in_dir(&dir.path().to_string_lossy().to_string());

        //then
        assert_eq!(dirs.len(), 1)
    }
}