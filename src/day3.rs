use regex::Regex;

fn sum_matches(input: &str) -> i32 {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    pattern.captures_iter(input)
        .map(|cap| {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            x * y
        })
        .sum()
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    sum_matches(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), 161);
    }
}
