use std::cmp::Ordering;

fn parse_numbers(number_lines: &str) -> Vec<usize> {
    number_lines
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

#[test]
fn test_parse_numbers() {
    let lines = "1
3
4
6";
    assert_eq!(vec![1, 3, 4, 6], parse_numbers(lines));
}

fn possible_sums(ns: &[usize]) -> Vec<usize> {
    let mut r = Vec::new();
    for i in ns {
        for j in ns {
            r.push(i + j);
        }
    }
    r
}

#[test]
fn test_possible_sums() {
    assert_eq!(vec![2, 3, 4, 3, 4, 5, 4, 5, 6], possible_sums(&[1, 2, 3]))
}

fn part1(numbers: &Vec<usize>, preamble: usize) -> usize {
    // println!("numbers: {:?}", numbers);
    for i in preamble..numbers.len() {
        let start = i - preamble;
        let end = i;
        let window = &numbers[start..end];
        // println!("i: {}, w: {:?}", i, window);
        let checksums = possible_sums(window);
        if !checksums.contains(&numbers[i]) {
            // println!("i: {}, not in checksums: {:?}", i, checksums);
            return numbers[i];
        }
    }
    0
}

#[test]
fn p1_test() {
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
    let expected = 127;
    assert_eq!(expected, part1(&parse_numbers(input), 5));
}

fn part2(numbers: &Vec<usize>, target: usize) -> usize {
    for i in 1..numbers.len() {
        // start at 1 because we need at least 2 numbers
        for window_size in 2..(i + 1) {
            let start = i + 1;
            let window = &numbers[(start - window_size)..start];
            match window.iter().sum::<usize>().cmp(&target) {
                Ordering::Less => continue,
                Ordering::Equal => {
                    return window.iter().min().unwrap() + window.iter().max().unwrap()
                }
                Ordering::Greater => break,
            }
        }
    }
    0 // ideally we should return a Result :)
}

fn main() {
    // given a seq of numbers
    // construct a moving window of 25 numbers
    // echo following number must be the sum of any two numbers in the window
    // first number which is not such a sum is the result
    //
    // will need a fn to generat all possible permutations of 25 n in the window
    //
    //
    // read & parse all lines into a Vec numbers
    // for n in 25..numbers.len()
    // make a slice of n-25..n
    // make all possible permuations
    // summ all tuples into a vec -> check_sum
    // check that check_sum contains numbers[n]
    // if not print && exit
    let input = include_str!("../input");
    let numbers = parse_numbers(input);

    println!("part 1 -> {}", part1(&numbers, 25)); // -> 144381670

    println!("part 2 -> {}", part2(&numbers, 144381670)); // -> 144381670
}
