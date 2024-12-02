fn basic_it<'a>(input: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    input.lines()
}

fn part1_impl<'a, I>(iter: I) -> i32
    where I: IntoIterator<Item = &'a str>
{
    let mut valid = 0;
    for line in iter {
        // Parse into i8 as you specified
        let vec: Vec<i8> = line.split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();
        
        // Create a difference vector using prefix scanning
        let diff: Vec<i8> = vec.windows(2)
            .map(|window| window[1] - window[0])
            .collect();

        // Panic if diff is empty, otherwise validate based on first difference's sign
        let line_valid = if diff.first().unwrap_or_else(|| panic!("Empty difference vector")) < &0
            { diff.iter().all(|&x| x >= -3 && x <= -1) } else
            { diff.iter().all(|&x| x >= 1 && x <= 3) };
        
        if line_valid {
            valid += 1;
        }
    }

    valid
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    part1_impl(basic_it(input))
}

#[cfg(test)]
mod tests {
    use super::*; // Import the `part2` function from the current module

    #[test]
    fn test_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part1(input), 2);
    }
}
