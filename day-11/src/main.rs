use std::fs;

fn parse_seats(seat_input: &str) -> Vec<Vec<char>> {
    seat_input
        .split('\n')
        .map(|l| l.chars().collect())
        .collect()
}

fn print_seat_grid(seat_grid: &Vec<Vec<char>>) {
    println!(
        "{}",
        seat_grid
            .iter()
            .map(|r| r.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn adjacent_seats(seat_grid: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<char> {
    let mut seats: Vec<char> = vec![];
    for row_idx in row.saturating_sub(1)..=row + 1 {
        if row_idx >= seat_grid.len() {
            continue;
        }
        let seat_row = &seat_grid[row_idx];
        for col_idx in col.saturating_sub(1)..=col + 1 {
            if col_idx >= seat_row.len() || (row_idx == row && col_idx == col) {
                continue;
            }
            seats.push(seat_row[col_idx]);
        }
    }
    seats
}

fn line_of_sight_seats(
    seat_grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    row_diff: isize,
    col_diff: isize,
) -> Vec<char> {
    let mut seats: Vec<char> = vec![];
    let mut row_idx = row;
    let mut col_idx = col;

    while row_idx < seat_grid.len() && col_idx < seat_grid[row].len() {
        if !(row_idx == row && col_idx == col) {
            let new_seat = seat_grid[row_idx][col_idx];
            seats.push(new_seat);
            if new_seat != '.' {
                return seats;
            }
        }
        let row_change = if row_diff.is_negative() {
            row_idx.checked_sub(row_diff.wrapping_abs() as usize)
        } else {
            Some(row_idx + row_diff as usize)
        };
        let col_change = if col_diff.is_negative() {
            col_idx.checked_sub(col_diff.wrapping_abs() as usize)
        } else {
            Some(col_idx + col_diff as usize)
        };
        if row_change.is_none() || col_change.is_none() {
            break;
        };

        row_idx = row_change.unwrap();
        col_idx = col_change.unwrap();
    }

    seats
}

fn visible_seats(seat_grid: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<char> {
    let steps: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    steps
        .iter()
        .flat_map(|(row_diff, col_diff)| {
            line_of_sight_seats(seat_grid, row, col, *row_diff, *col_diff)
        })
        .collect()
}

fn new_seat(seat: char, seats: Vec<char>, occupied_count: usize) -> char {
    let occupied = seats.iter().filter(|s| **s == '#').count();
    match (seat, occupied, occupied >= occupied_count) {
        ('L', 0, _) => '#',
        ('#', _, true) => 'L',
        _ => seat,
    }
}

fn run_simulation(seat_grid: Vec<Vec<char>>) -> (Vec<Vec<char>>, usize) {
    let mut new_seat_grid: Vec<Vec<char>> = vec![];
    let mut changes: usize = 0;

    for (row_idx, row) in seat_grid.iter().enumerate() {
        let mut row_vec = vec![];
        for (col_idx, col) in row.iter().enumerate() {
            let new_seat_char = new_seat(*col, adjacent_seats(&seat_grid, row_idx, col_idx), 4);
            if new_seat_char != *col {
                changes += 1;
            }
            row_vec.push(new_seat_char);
        }
        new_seat_grid.push(row_vec);
    }
    (new_seat_grid, changes)
}

fn simulate_occupied_seats(seat_grid: Vec<Vec<char>>) -> usize {
    let mut update_seat_grid = seat_grid;
    loop {
        let (changed_seat_grid, changes) = run_simulation(update_seat_grid.clone());
        update_seat_grid = changed_seat_grid;
        if changes == 0 {
            break;
        }
    }

    update_seat_grid
        .iter()
        .map(|r| r.iter().filter(|c| **c == '#').count())
        .sum::<usize>()
}

fn run_simulation_2(seat_grid: Vec<Vec<char>>) -> (Vec<Vec<char>>, usize) {
    let mut new_seat_grid: Vec<Vec<char>> = vec![];
    let mut changes: usize = 0;

    for (row_idx, row) in seat_grid.iter().enumerate() {
        let mut row_vec = vec![];
        for (col_idx, col) in row.iter().enumerate() {
            let new_seat_char = new_seat(*col, visible_seats(&seat_grid, row_idx, col_idx), 5);
            if new_seat_char != *col {
                changes += 1;
            }
            row_vec.push(new_seat_char);
        }
        new_seat_grid.push(row_vec);
    }
    (new_seat_grid, changes)
}

fn simulate_occupied_seats_2(seat_grid: Vec<Vec<char>>) -> usize {
    let mut update_seat_grid = seat_grid;

    loop {
        let (changed_seat_grid, changes) = run_simulation_2(update_seat_grid.clone());
        update_seat_grid = changed_seat_grid;
        if changes == 0 {
            break;
        }
    }

    update_seat_grid
        .iter()
        .map(|r| r.iter().filter(|c| **c == '#').count())
        .sum::<usize>()
}

fn main() {
    let input = fs::read_to_string("./day-11/input.txt").unwrap();

    let seat_grid = parse_seats(&input);
    println!("Part 1: {}", simulate_occupied_seats(seat_grid.clone()));

    println!("Part 2: {}", simulate_occupied_seats_2(seat_grid));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_1() {
        let sample = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let seat_grid = parse_seats(sample);
        assert_eq!(simulate_occupied_seats(seat_grid), 37);
    }

    #[test]
    fn test_sample_2() {
        let sample = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let seat_grid = parse_seats(sample);
        assert_eq!(simulate_occupied_seats_2(seat_grid), 26);
    }
}
