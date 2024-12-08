advent_of_code::solution!(2);

fn reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|lvl| lvl.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

fn safety(reports: Vec<Vec<i32>>) -> Vec<bool> {
    reports.iter().map(|report| report_safety(report)).collect()
}

fn dampened_safety(reports: Vec<Vec<i32>>) -> Vec<bool> {
    reports
        .iter()
        .map(|report| match report_safety(report) {
            true => true,
            false => {
                for i in 0..report.len() {
                    let mut report = report.clone();
                    let _ = report.remove(i);
                    if report_safety(&report) {
                        return true;
                    }
                }
                return false;
            }
        })
        .collect()
}

fn report_safety(report: &Vec<i32>) -> bool {
    match &report[0].cmp(&report[1]) {
        std::cmp::Ordering::Greater => report
            .windows(2)
            .map(|window| {
                let diff = window[0] - window[1];
                0 < diff && diff <= 3
            })
            .all(|safe| safe == true),
        std::cmp::Ordering::Less => report
            .windows(2)
            .map(|window| {
                let diff = window[1] - window[0];
                0 < diff && diff <= 3
            })
            .all(|safe| safe == true),
        std::cmp::Ordering::Equal => false,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = reports(input);
    let safety_report = safety(reports);
    Some(
        safety_report
            .iter()
            .filter(|report| **report == true)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = reports(input);
    let safety_report = dampened_safety(reports);
    Some(
        safety_report
            .iter()
            .filter(|report| **report == true)
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
