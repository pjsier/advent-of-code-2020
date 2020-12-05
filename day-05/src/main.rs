use std::fs;

fn calculate_seat_idx(num_seats: i64, ticket: &str) -> i64 {
    let mut seat_idx: i64 = 0;
    let mut search_len: i64 = num_seats;

    for ticket_char in ticket.chars() {
        search_len = ((search_len as f64) / 2.0).floor() as i64;
        if ticket_char == 'B' || ticket_char == 'R' {
            seat_idx += search_len;
        }
    }
    seat_idx
}

fn calculate_ticket_seat_id(num_rows: i64, num_cols: i64, ticket: &str) -> i64 {
    (calculate_seat_idx(
        num_rows,
        ticket
            .chars()
            .filter(|c| *c == 'F' || *c == 'B')
            .collect::<String>()
            .as_str(),
    ) * 8)
        + calculate_seat_idx(
            num_cols,
            ticket
                .chars()
                .filter(|c| *c == 'L' || *c == 'R')
                .collect::<String>()
                .as_str(),
        )
}

fn find_missing_seat_id(num_rows: i64, num_cols: i64, tickets: Vec<&str>) -> Option<i64> {
    let seat_ids: Vec<i64> = tickets
        .iter()
        .map(|ticket| calculate_ticket_seat_id(num_rows, num_cols, ticket))
        .collect();
    for row_idx in 1..(num_rows - 1) {
        for col_idx in 0..num_cols {
            let seat_id = (row_idx * 8) + col_idx;
            if !seat_ids.contains(&seat_id)
                && seat_ids.contains(&(seat_id - 1))
                && seat_ids.contains(&(seat_id + 1))
            {
                return Some(seat_id);
            }
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("./day-05/input.txt").unwrap();

    let max_idx = input
        .split('\n')
        .map(|ticket| calculate_ticket_seat_id(128, 8, ticket))
        .max()
        .unwrap();

    println!("Part 1: {}", max_idx);

    println!(
        "Part 2: {}",
        find_missing_seat_id(128, 8, input.split('\n').collect()).unwrap()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_1() {
        assert_eq!(calculate_seat_idx(128, "FBFBBFF"), 44);
        assert_eq!(calculate_seat_idx(8, "RLR"), 5);
        assert_eq!(calculate_ticket_seat_id(128, 8, "FBFBBFFRLR"), 357);
        assert_eq!(calculate_ticket_seat_id(128, 8, "BFFFBBFRRR"), 567);
        assert_eq!(calculate_ticket_seat_id(128, 8, "FFFBBBFRRR"), 119);
        assert_eq!(calculate_ticket_seat_id(128, 8, "BBFFBBFRLL"), 820);
    }
}
