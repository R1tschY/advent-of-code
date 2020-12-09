use aoc2020::{read_int_lines, read_lines};

fn parse_id(s: &str) -> u32 {
    s.chars()
        .map(|c| c == 'B' || c == 'R')
        .fold(0u32, |acc, v| (acc << 1) | v as u32)
}

fn calc1(input: &str) -> Option<u32> {
    let inputs = read_lines(input);

    inputs.iter().map(|s| parse_id(s)).max()
}

fn calc2(input: &str) -> Option<u64> {
    let inputs = read_lines(input);

    let all = inputs
        .iter()
        .map(|s| parse_id(s))
        .fold(vec![false; 1024], |mut acc, v| {
            acc[v as usize] = true;
            acc
        });

    let vec1 = all
        .iter()
        .enumerate()
        .filter(|(_i, v)| !**v)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    println!("{:?}", vec1);

    None
}

fn main() {
    println!("{:?}", calc1(include_str!("aoc05.txt")));
    println!("{:?}", calc2(include_str!("aoc05.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check1() {
        assert_eq!(parse_id("FBFBBFFRLR"), 357);
    }

    #[test]
    fn check2() {
        let input = "1721
979
366
299
675
1456";
        assert_eq!(calc2(input), Some(241861950));
    }
}
