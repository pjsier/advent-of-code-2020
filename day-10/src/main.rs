use std::cmp;
use std::collections::HashMap;
use std::fs;

fn adapter_chain_diffs(adapters: Vec<usize>, start: usize, end_diff: usize) -> Vec<usize> {
    let mut all_adapters: Vec<usize> = vec![start];
    all_adapters.extend(adapters);
    all_adapters.sort_unstable();

    let mut adapters = all_adapters[..]
        .windows(2)
        .map(|v| v[1].checked_sub(v[0]).unwrap())
        .collect::<Vec<usize>>();
    adapters.push(end_diff);
    adapters
}

fn sort_adapters(adapters: Vec<usize>, start: usize, end_diff: usize) -> Vec<usize> {
    let mut all_adapters: Vec<usize> = vec![start];
    all_adapters.extend(adapters);
    all_adapters.sort_unstable();

    let last_item = all_adapters[all_adapters.len() - 1];
    all_adapters.push(last_item + end_diff);

    all_adapters
}

fn count_arrangements(
    index: usize,
    map: &mut HashMap<usize, usize>,
    adapters: &Vec<usize>,
) -> usize {
    if index == adapters.len() - 1 {
        return 1;
    }

    if let Some(&count) = map.get(&index) {
        return count;
    }

    let count = adapters[index + 1..=cmp::min(index + 3, adapters.len() - 1)]
        .iter()
        .enumerate()
        .filter(|(_, a)| a.checked_sub(adapters[index]).unwrap_or(4) <= 3)
        .map(|(idx, _)| count_arrangements(index + 1 + idx, map, adapters))
        .sum();

    map.insert(index, count);

    count
}

fn main() {
    let input = fs::read_to_string("./day-10/input.txt").unwrap();

    let adapters: Vec<usize> = input
        .split('\n')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let diffs = adapter_chain_diffs(adapters.clone(), 0, 3);
    let diffs_1 = diffs.iter().filter(|v| **v == 1).count();
    let diffs_3 = diffs.iter().filter(|v| **v == 3).count();
    println!("Part 1: {}", diffs_1 * diffs_3);

    let sorted_adapters = sort_adapters(adapters, 0, 3);
    println!(
        "Part 2: {}",
        count_arrangements(0, &mut HashMap::new(), &sorted_adapters)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_1() {
        let sample = "16
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
        let adapters: Vec<usize> = sample
            .split('\n')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let diffs = adapter_chain_diffs(adapters, 0, 3);
        assert_eq!(diffs.iter().filter(|v| **v == 1).count(), 7);
        assert_eq!(diffs.iter().filter(|v| **v == 3).count(), 5);
    }

    #[test]
    fn test_sample_2() {
        let sample = "28
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
        let adapters: Vec<usize> = sample
            .split('\n')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let diffs = adapter_chain_diffs(adapters, 0, 3);
        assert_eq!(diffs.iter().filter(|v| **v == 1).count(), 22);
        assert_eq!(diffs.iter().filter(|v| **v == 3).count(), 10);
    }

    #[test]
    fn test_counts_sample_1() {
        let sample = "16
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
        let adapters: Vec<usize> = sample
            .split('\n')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let sorted_adapters = sort_adapters(adapters, 0, 3);
        assert_eq!(
            count_arrangements(0, &mut HashMap::new(), &sorted_adapters),
            8
        );
    }
    #[test]
    fn test_counts_sample_2() {
        let sample = "28
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
        let adapters: Vec<usize> = sample
            .split('\n')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let sorted_adapters = sort_adapters(adapters, 0, 3);
        assert_eq!(
            count_arrangements(0, &mut HashMap::new(), &sorted_adapters),
            19208
        );
    }
}
