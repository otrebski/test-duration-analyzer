use crate::model::TestSuite;
use clap::Parser;

mod loader;
mod model;
mod parser;
mod processing;

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
    let by_first_letter = processing::group_by_first_letter(test_suites);

    let groups = processing::divide_into_groups(args.count, by_first_letter);

    for group in &groups {
        let string: String = group.iter().map(|tbl| tbl.letter.clone()).collect();
        println!("=======================================");
        println!(
            "Group: {}: {}s",
            string,
            group.iter().map(|tbl| tbl.time).sum::<f32>().round()
        );
        group
            .iter()
            .for_each(|tbl| println!(" - {}: {}s", tbl.letter, tbl.time.round().abs()));
    }
    println!("=======================================");
    println!(
        "Total time: {}",
        groups
            .iter()
            .flatten()
            .map(|tbl| tbl.time)
            .sum::<f32>()
            .round()
            .abs()
    );
}
