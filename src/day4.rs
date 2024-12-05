
fn count_matches(x: &[u8]) -> i32 {
    let side = if x.len() == 11*9+10 { 10 } else { 140 };
    let spacing = side + 1;
    // assert_eq!(x.len(), 11*9+10);
    let mut count = 0;
    for r in 0..side {
        for c in 0..side {
            if r + 4 <= side && c + 4 <= side {
                let mut i = r * spacing + c;
                let xmas_match_r = true;
                let samx_match_r = true;
                let xmas_match_r = xmas_match_r && x[i] == b'X';
                let samx_match_r = samx_match_r && x[i] == b'S';
                i += spacing + 1;
                let xmas_match_r = xmas_match_r && x[i] == b'M';
                let samx_match_r = samx_match_r && x[i] == b'A';
                i += spacing + 1;
                let xmas_match_r = xmas_match_r && x[i] == b'A';
                let samx_match_r = samx_match_r && x[i] == b'M';
                i += spacing + 1;
                let xmas_match_r = xmas_match_r && x[i] == b'S';
                let samx_match_r = samx_match_r && x[i] == b'X';
                count += if xmas_match_r { 1 } else { 0 };
                count += if samx_match_r { 1 } else { 0 };
            }
            if r + 4 <= side && c >= 3 {
                let mut i = r * spacing + c;
                let xmas_match_l = true;
                let samx_match_l = true;
                let xmas_match_l = xmas_match_l && x[i] == b'X';
                let samx_match_l = samx_match_l && x[i] == b'S';
                i += spacing - 1;
                let xmas_match_l = xmas_match_l && x[i] == b'M';
                let samx_match_l = samx_match_l && x[i] == b'A';
                i += spacing - 1;
                let xmas_match_l = xmas_match_l && x[i] == b'A';
                let samx_match_l = samx_match_l && x[i] == b'M';
                i += spacing - 1;
                let xmas_match_l = xmas_match_l && x[i] == b'S';
                let samx_match_l = samx_match_l && x[i] == b'X';
                count += if xmas_match_l { 1 } else { 0 };
                count += if samx_match_l { 1 } else { 0 };
            }
            if r + 4 <= side {
                let mut i = r * spacing + c;
                let xmas_match_v = true;
                let samx_match_v = true;
                let xmas_match_v = xmas_match_v && x[i] == b'X';
                let samx_match_v = samx_match_v && x[i] == b'S';
                i += spacing;
                let xmas_match_v = xmas_match_v && x[i] == b'M';
                let samx_match_v = samx_match_v && x[i] == b'A';
                i += spacing;
                let xmas_match_v = xmas_match_v && x[i] == b'A';
                let samx_match_v = samx_match_v && x[i] == b'M';
                i += spacing;
                let xmas_match_v = xmas_match_v && x[i] == b'S';
                let samx_match_v = samx_match_v && x[i] == b'X';
                count += if xmas_match_v { 1 } else { 0 };
                count += if samx_match_v { 1 } else { 0 };
            }
            if c + 4 <= side {
                let mut i = r * spacing + c;
                let xmas_match_h = true;
                let samx_match_h = true;
                let xmas_match_h = xmas_match_h && x[i] == b'X';
                let samx_match_h = samx_match_h && x[i] == b'S';
                i += 1;
                let xmas_match_h = xmas_match_h && x[i] == b'M';
                let samx_match_h = samx_match_h && x[i] == b'A';
                i += 1;
                let xmas_match_h = xmas_match_h && x[i] == b'A';
                let samx_match_h = samx_match_h && x[i] == b'M';
                i += 1;
                let xmas_match_h = xmas_match_h && x[i] == b'S';
                let samx_match_h = samx_match_h && x[i] == b'X';
                count += if xmas_match_h { 1 } else { 0 };
                count += if samx_match_h { 1 } else { 0 };
            }
        }
    }
    count
}

fn count_xs(x: &[u8]) -> i32 {
    let side = if x.len() == 11*9+10 { 10 } else { 140 };
    let spacing = side + 1;
    let mut count = 0;
    for r in 0..side {
        for c in 0..side {
            // M.S
            // .A.
            // M.S
            if r + 3 <= side && c + 3 <= side {
                let nw = r * spacing + c;
                let ne = nw+2;
                let sw = nw+spacing + spacing;
                let se = ne + spacing + spacing;
                if x[nw+spacing+1] == b'A' {
                    let mas_match_r = x[nw] == b'M' && x[se] == b'S';
                    let sam_match_r = x[nw] == b'S' && x[se] == b'M';
                    let mas_match_l = x[ne] == b'M' && x[sw] == b'S';
                    let sam_match_l = x[ne] == b'S' && x[sw] == b'M';
                    count += if (mas_match_r || sam_match_r) && (mas_match_l || sam_match_l) { 1 } else { 0 }
                }
            }
        }
    }
    count
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    count_matches(input.as_bytes())
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    count_xs(input.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part1(input), 18);
    }
    #[test]
    fn test_part2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part2(input), 9);
    }
}
