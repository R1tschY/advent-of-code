use aoc2020::read_int_lines;

fn calc1(input: &str) -> Option<u64> {
    let inputs = read_int_lines(input).expect("invalid input");

    for i in &inputs {
        for j in &inputs {
            if i + j == 2020 {
                return Some(i * j);
            }
        }
    }

    None
}

fn calc2(input: &str) -> Option<u64> {
    let inputs = read_int_lines(input).expect("invalid input");

    for i in &inputs {
        for j in &inputs {
            for k in &inputs {
                if i + j + k == 2020 {
                    return Some(i * j * k);
                }
            }
        }
    }

    None
}

fn main() {
    println!("{:?}", calc1(include_str!("aoc01.txt")));
    println!("{:?}", calc2(include_str!("aoc01.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check1() {
        let input = "1721
979
366
299
675
1456";
        assert_eq!(calc1(input), Some(514579));
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
