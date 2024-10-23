use crate::model::{TestSuite, TimeByLetter};
use std::collections::BTreeMap;

pub fn divide_into_groups(
    group_count: u16,
    times_by_letters: Vec<TimeByLetter>,
) -> Vec<Vec<TimeByLetter>> {
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

fn duration(test_suites: &Vec<TestSuite>) -> f32 {
    test_suites.iter().map(|ts| ts.time).sum()
}

pub fn group_by_first_letter(vec: Vec<TestSuite>) -> Vec<TimeByLetter> {
    let mut groups: BTreeMap<char, Vec<TestSuite>> = BTreeMap::new();
    ('A'..='Z').for_each(|c| {
        groups.insert(c, Vec::new());
    });
    for item in vec {
        let first_letter = item
            .name
            .split('.')
            .last()
            .unwrap()
            .chars()
            .next()
            .unwrap_or('0');
        groups
            .entry(first_letter.clone())
            .or_insert_with(Vec::new)
            .push(item)
    }
    groups
        .iter()
        .map(|(letter, test_suites)| TimeByLetter::new(duration(test_suites), letter.clone()))
        .collect()
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn empty_duration() {
        //when
        let duration = super::duration(&vec![]);

        //then
        assert_eq!(duration, 0.0);
    }
    #[test]
    fn non_empty_duration() {
        //given
        let test_suite = TestSuite {
            name: String::new(),
            time: 1.0,
            test_cases: vec![],
        };

        //when
        let duration = super::duration(&vec![test_suite]);

        //then
        assert_eq!(duration, 1.0);
    }
}
