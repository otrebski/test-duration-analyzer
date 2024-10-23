use crate::model::{FilePath, TestSuite};
use quick_xml::de::from_str;
use std::fs;

pub fn file_to_report(path: &FilePath) -> Option<TestSuite> {
    let content = fs::read_to_string(&path.path)
        .map_err(|_| {
            eprintln!("Can't read content of file {}", path.path);
        })
        .ok()?;
    from_str::<TestSuite>(&content)
        .map_err(|_| eprintln!("Can't parse file {}", path.path))
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_file_to_report_success() {
        //given
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.xml");

        // Create a sample JSON file
        let mut file = File::create(&path).unwrap();
        let test_suite = r#"
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="scenario.SearchTest" tests="3" skipped="0" failures="0" errors="0" timestamp="2024-10-18T20:40:34" hostname="pudlo" time="39.218">
    <properties/>
    <testcase name="testSearchQuery" classname="scenario.SearchTest" time="16.274"/>
    <testcase name="testSearchRegex" classname="scenario.SearchTest" time="10.609"/>
    <testcase name="testSearchString" classname="scenario.SearchTest" time="11.391"/>
    <system-out><![CDATA[]]></system-out>
    <system-err><![CDATA[]]></system-err>
</testsuite>"#; // Adjust based on TestSuite structure
        writeln!(file, "{}", test_suite).unwrap();

        //when
        let result = file_to_report(&FilePath {
            path: path.to_string_lossy().into_owned(),
        });

        //then
        assert!(result.is_some()); // Check if we get a Some(TestSuite)
    }

    #[test]
    fn test_file_to_report_not_found() {
        //when
        let result = file_to_report(&FilePath {
            path: "non_existent_file.json".to_string(),
        });

        //then
        assert!(result.is_none()); // Expect None for non-existent file
    }

    #[test]
    fn test_file_to_report_invalid_xml() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("invalid.xml");

        // Create a sample invalid JSON file
        let mut file = File::create(&path).unwrap();
        writeln!(file, "invalid xml").unwrap();

        let result = file_to_report(&FilePath {
            path: path.to_string_lossy().into_owned(),
        });
        assert!(result.is_none()); // Expect None for invalid JSON
    }
}
