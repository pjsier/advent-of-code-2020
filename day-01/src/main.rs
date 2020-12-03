use std::fs;

const ENTRY_SUM: i64 = 2020;

fn number_match_two(entries: Vec<i64>) -> Option<i64> {
    entries
        .iter()
        .filter_map(|&i| match entries.iter().find(|&x| i + x == ENTRY_SUM) {
            Some(val) => Some(i * *val),
            None => None,
        })
        .next()
}

fn number_match_three(entries: Vec<i64>) -> Option<i64> {
    for x in entries.iter() {
        for y in entries.iter() {
            for z in entries.iter() {
                if x + y + z == ENTRY_SUM {
                    return Some(x * y * z);
                }
            }
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("./day-01/input.txt").unwrap();
    let entries: Vec<i64> = input
        .split('\n')
        .map(|v| v.parse::<i64>())
        .filter_map(Result::ok)
        .collect();

    println!("Part 1 {}", number_match_two(entries.clone()).unwrap());
    println!("Part 2 {}", number_match_three(entries).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_match_two() {
        let sample: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(number_match_two(sample).unwrap(), 514579);
    }

    #[test]
    fn test_number_match_three() {
        let sample: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(number_match_three(sample).unwrap(), 241861950);
    }
}
