use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(_input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    // Find all matches and extract the numbers
    for cap in re.captures_iter(_input) {
        let x: u32 = cap[1].parse().unwrap();
        let y: u32 = cap[2].parse().unwrap();
        sum += x * y;
    }

    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    // 首先找到所有的 don't()...do() 对
    let mut result = String::from(_input);
    println!("{}", result.len());

    // 处理所有的 don't() 和 do() 指令
    loop {
        let dont_pos = result.find("don't()");
        match dont_pos {
            Some(d1) => {
                let do_pos = result.find("do()");
                match do_pos {
                    Some(d2) => {
                        if d1 < d2 {
                            result = result[..d1].to_string() + &result[d2 + 4..];
                        } else {
                            result = result[..d2].to_string() + &result[d2 + 4..];
                        }
                    }
                    None => {
                        result = result[..d1].to_string();
                        break;
                    }
                }
            }
            None => break,
        }
    }
    // 根据最后的状态处理 mul 指令
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    for cap in re.captures_iter(&result) {
        let x: u32 = cap[1].parse().unwrap();
        let y: u32 = cap[2].parse().unwrap();
        sum += x * y;
    }

    Some(sum)
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
