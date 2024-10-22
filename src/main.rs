use std::collections::BTreeMap;
use clap::Parser;
use crate::model::{TestSuite, TimeByLetter};

mod model;
mod loader;
mod parser;

#[derive(Parser, Debug)]
#[command(name = "command ...")]
struct Args {
    /// Number of groups
    #[arg(short, long, default_value_t = 5)]
    count: u16,

    /// List of paths with JUNIT reports
    #[arg(default_value = ".")]
    paths: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let vec = loader::list_xml_files_in_dirs(args.paths);
    let test_suites: Vec<TestSuite> = vec
        .iter()
        .filter_map(|file_path| parser::file_to_report(file_path))
        .collect();
    let by_first_letter = group_by_first_letter(test_suites);

    let groups = divide_into_groups(args.count, by_first_letter);

    for group in &groups {
        let string: String = group.iter().map(|tbl| tbl.letter.clone()).collect();
        println!("=======================================");
        println!("Group: {}: {}s", string, group.iter().map(|tbl| tbl.time).sum::<f32>().round());
        group.iter().for_each(|tbl| println!(" - {}: {}s", tbl.letter, tbl.time.round().abs()));
    }
    println!("=======================================");
    println!("Total time: {}", groups.iter().flatten().map(|tbl| tbl.time).sum::<f32>().round().abs());
}

fn duration(test_suites: &Vec<TestSuite>) -> f32 {
    test_suites.iter().map(|ts| ts.time).sum()
}


fn divide_into_groups(group_count: u16, times_by_letters: Vec<TimeByLetter>) -> Vec<Vec<TimeByLetter>> {
    let total_time: f32 = times_by_letters.iter().map(|t| t.time).sum();
    let target_duration = total_time / group_count as f32;

    let mut result: Vec<Vec<TimeByLetter>> = Vec::new();
    let mut current_group: Vec<TimeByLetter> = Vec::new();
    let mut current_sum = 0.0;

    for time_by_letter in times_by_letters {
        if time_by_letter.time > target_duration {
            // If the time is greater than the target duration, add it as a new group
            if !current_group.is_empty() {
                result.push(current_group);
                current_group = Vec::new();
                current_sum = 0.0;
            }
            result.push(vec![time_by_letter]);
        } else {
            // Otherwise, try to add it to the current group
            if current_sum + time_by_letter.time > target_duration && !current_group.is_empty() {
                result.push(current_group);
                current_group = Vec::new();
                current_sum = 0.0;
            }
            current_sum += time_by_letter.time;
            current_group.push(time_by_letter);
        }
    }

    // Push any remaining group
    if !current_group.is_empty() {
        result.push(current_group);
    }

    result
}
fn group_by_first_letter(vec: Vec<TestSuite>) -> Vec<TimeByLetter> {
    let mut groups: BTreeMap<char, Vec<TestSuite>> = BTreeMap::new();
    ('A'..='Z').for_each(|c| { groups.insert(c, Vec::new()); });
    for item in vec {
        let first_letter = item.name.split('.').last().unwrap().chars().next().unwrap_or('0');
        groups.entry(first_letter.clone()).or_insert_with(Vec::new).push(item)
    }
    groups
        .iter()
        .map(|(letter, test_suites)| TimeByLetter::new(duration(test_suites), letter.clone()))
        .collect()
}
