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

fn is_valid_vec(nums: &Vec<i32>, lo: i32, hi: i32) -> bool {
    // Less than 3 elements is always valid
    if nums.len() < 3 {
        return true;
    }

    // Try removing no element first
    if is_valid_sequence(nums, lo, hi) {
        return true;
    }

    // Try removing each element
    for skip_index in 0..nums.len() {
        let mut modified_vec = Vec::new();
        
        // Construct a new vec skipping the current element
        for (i, &num) in nums.iter().enumerate() {
            if i != skip_index {
                modified_vec.push(num);
            }
        }

        // If this modified sequence is valid, return true
        if is_valid_sequence(&modified_vec, lo, hi) {
            return true;
        }
    }

    false
}

fn is_valid_sequence(nums: &Vec<i32>, lo: i32, hi: i32) -> bool {
    // If less than 2 elements, it's valid by definition
    if nums.len() < 2 {
        return true;
    }

    // Check differences between consecutive elements
    for window in nums.windows(2) {
        let diff = window[1] - window[0];
        
        // Check if the difference is in the specified range
        if diff < lo || diff > hi {
            return false;
        }
    }

    true
}

fn part2_impl<'a, I>(iter: I) -> i32
    where I: IntoIterator<Item = &'a str>
{
    let mut valid = 0;
    for line in iter {
        // Parse into i8 as you specified
        let vec: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();
        if is_valid_vec(&vec, -3, -1) {
            valid += 1;
        } else if is_valid_vec(&vec, 1, 3) {
            valid += 1;
        }
    }

    valid
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    part2_impl(basic_it(input))
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

    #[test]
    fn test_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part2(input), 4);
    }

    #[test]
    fn test_part2d() {
        let input = "1 3 2 4 5";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn test_part2e() {
        let input = "8 6 4 4 1";
        assert_eq!(part2(input), 1);
    }
}
