use aoc2020::read_binary_map;

// fn print_map(map: &Vec<Vec<bool>>, i: usize, k: usize) {
//     let x = map
//         .iter()
//         .enumerate()
//         .map(|(ii, line)| {
//             line.iter()
//                 .enumerate()
//                 .map(|(ki, b)| {
//                     if ii == i && ki == k {
//                         if *b {
//                             'X'
//                         } else {
//                             'O'
//                         }
//                     } else {
//                         if *b {
//                             '#'
//                         } else {
//                             '.'
//                         }
//                     }
//                 })
//                 .collect::<String>()
//         })
//         .collect::<Vec<String>>()
//         .join("\n");
//     println!("{}", x);
// }

fn calc1(input: &str) -> Option<usize> {
    let inputs = read_binary_map(input, '#');
    let score = calc_score(&inputs, 1, 3);
    Some(score)
}

fn calc_score(inputs: &Vec<Vec<bool>>, step_i: usize, step_k: usize) -> usize {
    let cols = inputs[0].len();

    (0usize..)
        .map(|i| (i * step_i, (i * step_k) % cols))
        .take_while(|(i, _k)| *i < inputs.len())
        .filter(|(i, k)| inputs[*i][*k])
        .count()
}

fn calc2(input: &str) -> usize {
    let inputs = read_binary_map(input, '#');

    [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|(si, sk)| calc_score(&inputs, *si, *sk))
        .fold(1, |a, i| a * i)
}

fn main() {
    println!("{:?}", calc1(include_str!("aoc03.txt")));
    println!("{:?}", calc2(include_str!("aoc03.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check1() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        assert_eq!(calc1(input), Some(7));
    }

    #[test]
    fn check2() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        assert_eq!(calc2(input), 336);
    }
}
