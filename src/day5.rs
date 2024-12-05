use std::collections::{HashMap, HashSet};
// use std::vec::Vec;

fn parse_input(input: &str) -> i32 {
    // Split the input into two sections based on the double newline
    let mut sections = input.split("\n\n");
    let edges_section = sections.next().expect("Missing edges section");
    let sequences_section = sections.next().expect("Missing sequences section");

    // Parse the edges section into a HashMap
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for line in edges_section.lines() {
        let mut parts = line.split('|');
        let left = parts
            .next()
            .and_then(|x| x.parse::<i32>().ok())
            .expect("Invalid left value in edges section");
        let right = parts
            .next()
            .and_then(|x| x.parse::<i32>().ok())
            .expect("Invalid right value in edges section");

        graph.entry(left).or_default().push(right);
    }

    let mut acc = 0i32;

    for line in sequences_section.lines() {
        let mut visited: HashSet<i32> = HashSet::new();
        let sequence: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        let mut ok = true;

        for &x in &sequence {
            if !visited.insert(x) {
                ok = false; // x is already visited
                break;
            }
            let mut traverse_visited: HashSet<i32> = HashSet::new();
            // Recursively traverse from x in the graph and ensure no connected nodes are visited
            if !traverse_and_check(x, &graph, &mut visited, &mut traverse_visited) {
                ok = false;
                break;
            }
        }

        if ok {
            acc += sequence[sequence.len() / 2]; // Add the middle element of the sequence
        }
    }

    acc
}

fn traverse_and_check(node: i32, graph: &HashMap<i32, Vec<i32>>, match_set: &mut HashSet<i32>, visited: &mut HashSet<i32>) -> bool {
    if !visited.insert(node) {
        return true;
    }
    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if match_set.contains(&neighbor) {
                return false; // Neighbor is already visited
            }
            // Recursively check neighbors
            if !traverse_and_check(neighbor, graph, match_set, visited) {
                return false;
            }
        }
    }
    true
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    parse_input(input)
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(part1(input), 143);
    }
    // #[test]
    // fn test_part2() {
    //     let input = "MMMSXXMASM
    //     assert_eq!(part2(input), 9);
    // }
}
