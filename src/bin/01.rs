advent_of_code::solution!(1);

fn lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|ch| ch.parse::<i32>().ok())
            .collect();
        left.push(nums[0]);
        right.push(nums[1]);
    }
    (left, right)
}

fn sorted_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|ch| ch.parse::<i32>().ok())
            .collect();
        let left_pos = left.binary_search(&nums[0]).unwrap_or_else(|e| e);
        let right_pos = right.binary_search(&nums[1]).unwrap_or_else(|e| e);
        left.insert(left_pos, nums[0]);
        right.insert(right_pos, nums[1]);
    }
    (left, right)
}

fn distances(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .collect()
}

fn similarity(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut similarities: Vec<i32> = vec![];

    for el in left {
        let appearances: i32 = right
            .iter()
            .filter(|e| **e == el)
            .collect::<Vec<&i32>>()
            .len()
            .try_into()
            .unwrap();
        similarities.push(appearances * el);
    }
    similarities
}

pub fn part_one(input: &str) -> Option<u32> {
    let lists = sorted_lists(input);
    let distances = distances(lists.0, lists.1);
    let sum: i32 = distances.iter().sum();
    Some(sum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lists = lists(input);
    let similarities = similarity(lists.0, lists.1);
    let sum: i32 = similarities.iter().sum();
    Some(sum.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
