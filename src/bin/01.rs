advent_of_code::solution!(1);

pub fn part_one(_input: &str) -> Option<u32> {
    let lines = _input.lines();

    let mut nums_left = vec![];
    let mut nums_right = vec![];
    lines.for_each(|line| {
        let (left, right) = line.split_once(' ').unwrap();
        let left = left
            .split_whitespace()
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let right = right
            .split_whitespace()
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        nums_left.push(left);
        nums_right.push(right);
    });

    nums_left.sort();
    nums_right.sort();

    let mut sum = 0;

    for i in 0..nums_left.len() {
        sum += nums_left[i].abs_diff(nums_right[i]);
    }

    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let lines = _input.lines();

    let mut nums_left = vec![];
    let mut nums_right = vec![];
    lines.for_each(|line| {
        let (left, right) = line.split_once(' ').unwrap();
        let left = left
            .split_whitespace()
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let right = right
            .split_whitespace()
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        nums_left.push(left);
        nums_right.push(right);
    });

    let mut sum = 0;

    nums_left.iter().for_each(|&ele| {
        let sum_right = nums_right.iter().filter(|&&e| e == ele).count() as u32;
        sum += sum_right * ele;
    });
    Some(sum)
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
