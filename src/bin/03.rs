use regex::Regex;

advent_of_code::solution!(3);

const MUL_REGEX: &str = r#"mul\(\d{1,3},\d{1,3}\)"#;

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(MUL_REGEX).unwrap();
    let result = regex
        .find_iter(input)
        .map(|mul| {
            let nums: Vec<u32> = mul
                .as_str()
                .replace("mul(", "")
                .replace(")", "")
                .split(",")
                .filter_map(|num| num.parse::<u32>().ok())
                .collect();
            nums.iter().fold(1, |t, c| t * c)
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
