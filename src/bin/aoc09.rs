use aoc2020::read_int_lines;

fn is_valid(v: u64, values: &[u64]) -> bool {
    for i in values.iter() {
        for j in values.iter() {
            if i + j == v && i != j {
                return true;
            }
        }
    }

    false
}

fn calc1(input: &str, pre: usize) -> Option<u64> {
    let inputs = read_int_lines(input).expect("invalid input");

    for i in pre..inputs.len() {
        let value = inputs[i];
        if !is_valid(value, &inputs[(i - pre)..i]) {
            return Some(value);
        }
    }

    None
}

fn calc2(input: &str, value: u64) -> Option<u64> {
    let inputs = read_int_lines(input).expect("invalid input");

    for i in 0..inputs.len() {
        let mut acc = 0;
        for (k, j) in inputs[i..].iter().enumerate() {
            acc += j;
            if acc == value {
                let c = i + k + 1;
                let vec = inputs[i..c].iter().copied().collect::<Vec<u64>>();
                return Some(vec.iter().min().unwrap() + vec.iter().max().unwrap());
            } else if acc > value {
                break;
            }
        }
    }

    None
}

fn main() {
    let value = calc1(include_str!("aoc09.txt"), 25).unwrap();
    println!("{:?}", value);

    println!("{:?}", calc2(include_str!("aoc09.txt"), value));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check1() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(calc1(input, 5), Some(127));
    }

    #[test]
    fn check2() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(calc2(input, calc1(input, 5).unwrap()), Some(62));
    }
}
