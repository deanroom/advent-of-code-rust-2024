advent_of_code::solution!(2);

pub fn part_one(_input: &str) -> Option<u32> {
    let reports = _input
        .lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            // Check if numbers are increasing
            let is_increasing = numbers
                .windows(2)
                .all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3);

            // Check if numbers are decreasing
            let is_decreasing = numbers
                .windows(2)
                .all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3);
            Some(is_increasing || is_decreasing)
        })
        .collect::<Vec<Option<bool>>>();
    Some(reports.iter().filter(|x| x.unwrap()).count() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let reports = _input
        .lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            if !is_safe(&numbers) {
                for i in 0..numbers.len() {
                    let mut modified_numbers = numbers.clone();
                    modified_numbers.remove(i);
                    if is_safe(&modified_numbers) {
                        return Some(true);
                    }
                }
                return Some(false);
            }
            Some(true)
        })
        .collect::<Vec<Option<bool>>>();
    Some(reports.iter().filter(|x| x.unwrap()).count() as u32)
}

fn is_safe(numbers: &[i32]) -> bool {
    let is_increasing = numbers
        .windows(2)
        .all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3);

    // Check if numbers are decreasing
    let is_decreasing = numbers
        .windows(2)
        .all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3);
    is_increasing || is_decreasing
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
