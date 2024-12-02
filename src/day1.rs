use std::collections::{HashMap, HashSet};
use bit_set::BitSet;

fn split_once(s: &str, delimiter: char) -> Option<(&str, &str)> {
    if let Some(index) = s.find(delimiter) {
        Some((&s[..index], &s[index + delimiter.len_utf8()..]))
    } else {
        None
    }
}

pub fn part1_fallback(input: &str) -> i32 {
    // Parse the input into rows
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();


    // Iterate through each line and split into two columns
    for line in input.split('\n') {
        if let Some((first, second)) = split_once(line, ' ') {
            // Convert the first and second parts into integers
            if let (Ok(a), Ok(b)) = (
                first.parse::<i32>(),
                second.trim().parse::<i32>(),
            ) {
                col1.push(a);
                col2.push(b);
            }
        }
    }

    // Sort the columns
    col1.sort_unstable();
    col2.sort_unstable();

    // Calculate the sum of absolute differences
    col1.iter()
        .zip(col2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    // fastpath length per line = 10 digits for numbers, 3 spaces, 1 nl = 14
    // no nl for end
    let never_fastpath = (input.len() + 1) % 14 != 0;
    if never_fastpath {
        return part1_fallback(input);
    }
    return part1_fallback(input);
}

pub fn part2_fallback(input: &str) -> i64 {
    // Create hashmaps to count occurrences in each column
    let mut col1: HashMap<i32, i32> = HashMap::new();
    let mut col2: HashMap<i32, i32> = HashMap::new();

    // Iterate through each line and split into two columns
    for line in input.lines() {
        if let Some((first, second)) = split_once(line, ' ') {
            // Convert the first and second parts into integers
            if let (Ok(a), Ok(b)) = (
                first.parse::<i32>(),
                second.trim().parse::<i32>(),
            ) {
                // Increment occurrences of a in col1
                *col1.entry(a).or_insert(0) += 1;
                // Increment occurrences of b in col2
                *col2.entry(b).or_insert(0) += 1;
            }
        }
    }

    // Create an i64 counter for metric
    let mut total_similarity: i64 = 0;

    // Iterate over col1 hashmap
    for (key, count1) in &col1 {
        // Get single key similarity
        // Multiply occurrences in col1 by occurrences in col2 (default 0) and multiply by the key value
        let count2 = col2.get(key).unwrap_or(&0);
        total_similarity += *key as i64 * *count1 as i64 * *count2 as i64;
    }

    total_similarity
}

pub fn part2_fast(input: &str) -> i64 {
    // Create hashmaps to count occurrences in each column
    let mut dup_a: HashMap<i32, i16> = HashMap::with_capacity(100);
    let mut dup_b: HashMap<i32, i16> = HashMap::with_capacity(100);
    let mut bs_a = BitSet::with_capacity(100000);
    let mut bs_b = BitSet::with_capacity(100000);
    let mut match_ab: HashSet<i32> = HashSet::with_capacity(100);

    for chunk in input.as_bytes().chunks(14) {
        let b0 = (chunk[8+0] - b'0') as i32 * 10000;
        let b1 = (chunk[8+1] - b'0') as i32 * 1000;
        let b2 = (chunk[8+2] - b'0') as i32 * 100;
        let b3 = (chunk[8+3] - b'0') as i32 * 10;
        let b4 = (chunk[8+4] - b'0') as i32 * 1;

        let a0 = (chunk[0+0] - b'0') as i32 * 10000;
        let a1 = (chunk[0+1] - b'0') as i32 * 1000;
        let a2 = (chunk[0+2] - b'0') as i32 * 100;
        let a3 = (chunk[0+3] - b'0') as i32 * 10;
        let a4 = (chunk[0+4] - b'0') as i32 * 1;
        let a = a0+a1+a2+a3+a4;
        let b = b0+b1+b2+b3+b4;
        if bs_a.contains(a as usize) {
            *dup_a.entry(a).or_insert(1) += 1;
        }
        bs_a.insert(a as usize);
        if bs_a.contains(b as usize) {
            match_ab.insert(b);
        }
        if bs_b.contains(b as usize) {
            *dup_b.entry(b).or_insert(1) += 1;
        }
        bs_b.insert(b as usize);
        if bs_b.contains(a as usize) {
            match_ab.insert(a);
        }
    }

    // Create an i64 counter for metric
    let mut total_similarity: i64 = 0;

    for key in &match_ab {
        let count_a = *dup_a.get(key).unwrap_or(&1i16) as i64;
        let count_b = *dup_b.get(key).unwrap_or(&1i16) as i64;
        total_similarity += count_a * count_b * (*key as i64);
    }

    total_similarity
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i64 {
    // fastpath length per line = 10 digits for numbers, 3 spaces, 1 nl = 14
    // no nl for end
    let never_fastpath = (input.len() + 1) % 14 != 0;
    if never_fastpath {
        return part2_fallback(input);
    }
    return part2_fast(input);
}

#[cfg(test)]
mod tests {
    use super::*; // Import the `part2` function from the current module

    #[test]
    fn test_part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part2(input), 31);
    }
}
