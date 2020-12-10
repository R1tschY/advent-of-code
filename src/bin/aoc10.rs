use aoc2020::read_int_lines;
use std::collections::HashMap;

fn calc1(input: &str) -> Option<usize> {
    let mut inputs = read_int_lines(input).expect("invalid input");

    inputs.sort();
    let x: HashMap<u64, usize> = inputs
        .iter()
        .scan(0u64, |state, v| {
            let new = *v - *state;
            *state = *v;
            Some(new as u64)
        })
        .fold(HashMap::new(), |mut acc, v| {
            if let Some(existing) = acc.get_mut(&v) {
                *existing += 1usize;
            } else {
                acc.insert(v, 1usize);
            }

            acc
        });

    println!(
        "({}x{})",
        *x.get(&1).unwrap_or(&0),
        (*x.get(&3).unwrap_or(&0) + 1)
    );
    Some((*x.get(&1).unwrap_or(&0)) * (*x.get(&3).unwrap_or(&0) + 1))
}

fn calc2(input: &str) -> Option<usize> {
    let mut inputs = read_int_lines(input).expect("invalid input");
    inputs.sort();

    let mut x: HashMap<u64, usize> = inputs.iter().map(|i| (*i, 0)).collect();

    inc(&mut x, 1, 1);
    inc(&mut x, 2, 1);
    inc(&mut x, 3, 1);

    for v in &inputs {
        let i = *x.get(&v).unwrap();

        inc(&mut x, v + 1, i);
        inc(&mut x, v + 2, i);
        inc(&mut x, v + 3, i);
    }

    x.get(&inputs[inputs.len() - 1]).copied()
}

fn inc(x: &mut HashMap<u64, usize>, key: u64, i: usize) {
    if let Some(v) = x.get_mut(&key) {
        *v += i;
    }
}

fn main() {
    println!("{:?}", calc1(include_str!("aoc10.txt")));
    println!("{:?}", calc2(include_str!("aoc10.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check1() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(calc1(input), Some(7 * 5));
    }

    #[test]
    fn check12() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(calc1(input), Some(22 * 10));
    }

    #[test]
    fn check2() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(calc2(input), Some(8));
    }

    #[test]
    fn check22() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(calc2(input), Some(19208));
    }
}
