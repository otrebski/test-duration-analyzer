use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "testsuite")]
pub struct TestSuite {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@time")]
    pub time: f32,

    #[serde(rename = "testcase")]
    pub test_cases: Vec<TestCase>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TestCase {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@time")]
    pub time: f32,

    #[serde(rename = "@classname")]
    pub classname: String,
}

#[derive(Debug, Clone)] //PartialEq
pub struct TimeByLetter {
    pub time: f32,
    pub letter: char,
}
impl TimeByLetter {
    pub fn new(time: f32, letter: char) -> Self {
        TimeByLetter { time, letter }
    }
}

#[derive(Debug)]
pub struct FilePath {
    pub path: String,
}

// impl PartialOrd for TimeByLetter {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.letter.cmp(&other.letter))
//     }
// }
