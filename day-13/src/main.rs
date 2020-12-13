use std::cmp::Ordering;
use std::fs;

fn parse_notes(notes: &str) -> (i128, Vec<i128>) {
    let split_notes: Vec<&str> = notes.split('\n').collect();
    let earliest_time: i128 = split_notes[0].parse().unwrap();
    let buses: Vec<i128> = split_notes[1]
        .split(',')
        .filter(|b| *b != "x")
        .map(|b| b.parse::<i128>().unwrap())
        .collect();
    (earliest_time, buses)
}

fn parse_notes_index(notes: &str) -> Vec<i128> {
    let split_notes: Vec<&str> = notes.split('\n').collect();
    let buses: Vec<i128> = split_notes[1]
        .split(',')
        .map(|b| {
            if b == "x" {
                -1 as i128
            } else {
                b.parse::<i128>().unwrap()
            }
        })
        .collect();
    buses
}

// Chinese remainder theorem from Rosetta code
// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i128, n: i128) -> Option<i128> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i128], modulii: &[i128]) -> Option<i128> {
    let prod = modulii.iter().product::<i128>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p;
    }

    Some(sum % prod)
}

fn earliest_bus(earliest: i128, buses: Vec<i128>) -> i128 {
    let bus = buses
        .iter()
        .map(|b| (b, b * (earliest as f64 / *b as f64).ceil() as i128))
        .fold((0 as i128, 0 as i128) as (i128, i128), |acc, b| {
            if acc.0 == 0 {
                (*b.0, b.1)
            } else {
                match (acc.1 - earliest).cmp(&(b.1 - earliest)) {
                    Ordering::Less | Ordering::Equal => acc,
                    Ordering::Greater => (*b.0, b.1),
                }
            }
        });
    bus.0 * (bus.1 - earliest)
}

fn earliest_list_timestamp(buses: Vec<i128>) -> i128 {
    let buses_indexes: Vec<(i128, i128)> = buses
        .iter()
        .enumerate()
        .filter(|(_, b)| **b >= 0)
        .map(|(idx, b)| (idx as i128, *b))
        .collect();
    let modulii: Vec<i128> = buses_indexes.iter().map(|(_, b)| *b).collect();
    let residues: Vec<i128> = buses_indexes.iter().map(|(idx, _)| *idx).collect();
    modulii.iter().product::<i128>() - chinese_remainder(&residues, &modulii).unwrap()
}

fn main() {
    let input = fs::read_to_string("./day-13/input.txt").unwrap();

    let (earliest_time, buses) = parse_notes(&input);
    println!("Part 1: {}", earliest_bus(earliest_time, buses));

    let buses = parse_notes_index(&input);
    println!("Part 2: {}", earliest_list_timestamp(buses));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_1() {
        let sample = "939
7,13,x,x,59,x,31,19";
        let (earliest_time, buses) = parse_notes(sample);
        assert_eq!(earliest_bus(earliest_time, buses), 295);
    }

    #[test]
    fn test_sample_2() {
        let sample = parse_notes_index("1\n17,x,13,19");
        assert_eq!(earliest_list_timestamp(sample), 3417);
        let sample = parse_notes_index("1\n1789,37,47,1889");
        assert_eq!(earliest_list_timestamp(sample), 1202161486);
    }
}
