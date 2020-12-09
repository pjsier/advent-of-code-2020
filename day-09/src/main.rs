use std::cmp::Ordering;
use std::fs;

fn is_sum_of_two(num: usize, previous: Vec<usize>) -> bool {
    previous
        .iter()
        .any(|x| previous.iter().any(|y| x != y && x + y == num))
}

fn first_non_sum(numbers: Vec<usize>, preamble: usize) -> usize {
    numbers
        .iter()
        .enumerate()
        .skip(preamble)
        .filter(|(idx, n)| !is_sum_of_two(**n, numbers[idx - preamble..*idx].to_vec()))
        .map(|(_, n)| *n)
        .take(1)
        .next()
        .unwrap()
}

// Numbers is a portion of numbers starting at a sliced index
fn contiguous_range_adds_to_num(numbers: Vec<usize>, check_num: usize) -> Option<usize> {
    let mut total = 0;
    for (idx, num) in numbers.iter().enumerate() {
        total += num;
        match total.cmp(&check_num) {
            Ordering::Greater => return None,
            Ordering::Equal => {
                let contiguous_range: Vec<usize> = numbers[..=idx].iter().copied().collect();
                return Some(
                    contiguous_range.iter().min().unwrap() + contiguous_range.iter().max().unwrap(),
                );
            }
            _ => {}
        };
    }
    None
}

fn find_non_sum_range(numbers: Vec<usize>, preamble: usize) -> usize {
    let first_num_non_sum = first_non_sum(numbers.clone(), preamble);
    (0..numbers.len())
        .filter_map(|idx| contiguous_range_adds_to_num(numbers[idx..].to_vec(), first_num_non_sum))
        .take(1)
        .next()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("./day-09/input.txt").unwrap();

    let numbers: Vec<usize> = input
        .split('\n')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    println!("Part 1: {}", first_non_sum(numbers.clone(), 25));

    println!("Part 2: {}", find_non_sum_range(numbers, 25));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_1() {
        let sample = "35
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
        let numbers: Vec<usize> = sample
            .split('\n')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        assert_eq!(first_non_sum(numbers, 5), 127);
    }

    #[test]
    fn test_sample_2() {
        let sample = "35
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
        let numbers: Vec<usize> = sample
            .split('\n')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        assert_eq!(find_non_sum_range(numbers, 5), 62);
    }
}
