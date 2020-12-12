use std::fs;

fn rotate(dir: i32, turn: i32) -> i32 {
    let new_dir = dir + turn;
    if new_dir > 359 {
        new_dir - 360
    } else if new_dir < 0 {
        new_dir + 360
    } else {
        new_dir
    }
}

fn handle_instruction(
    instruction: &str,
    position: (i32, i32),
    direction: i32,
) -> ((i32, i32), i32) {
    let mut pos = position;
    let mut dir = direction;
    let command = instruction.chars().take(1).next().unwrap();
    let amount = instruction
        .chars()
        .skip(1)
        .map(String::from)
        .collect::<Vec<String>>()
        .join("")
        .parse::<i32>()
        .unwrap();

    match command {
        'N' => pos.1 += amount,
        'S' => pos.1 -= amount,
        'E' => pos.0 += amount,
        'W' => pos.0 -= amount,
        'R' => dir = rotate(dir, amount),
        'L' => dir = rotate(dir, -amount),
        'F' => {
            pos = match dir {
                0 => (pos.0, pos.1 + amount),
                90 => (pos.0 + amount, pos.1),
                180 => (pos.0, pos.1 - amount),
                270 => (pos.0 - amount, pos.1),
                _ => pos,
            }
        }
        _ => {}
    };

    (pos, dir)
}

fn run_instructions(instructions: &str, start: (i32, i32), direction: i32) -> i32 {
    let mut pos = start;
    let mut dir = direction;

    for instruction in instructions.split('\n') {
        let (new_pos, new_dir) = handle_instruction(instruction, pos, dir);
        pos = new_pos;
        dir = new_dir;
    }

    pos.0.abs() + pos.1.abs()
}

fn rotate_waypoint(waypoint: (i32, i32), degrees: i32) -> (i32, i32) {
    match degrees {
        90 | -270 => (-waypoint.1, waypoint.0),
        180 | -180 => (-waypoint.0, -waypoint.1),
        -90 | 270 => (waypoint.1, -waypoint.0),
        _ => waypoint,
    }
}

fn handle_instruction_waypoint(
    instruction: &str,
    position: (i32, i32),
    waypoint: (i32, i32),
) -> ((i32, i32), (i32, i32)) {
    let mut pos = position;
    let mut way = waypoint;
    let command = instruction.chars().take(1).next().unwrap();
    let amount = instruction
        .chars()
        .skip(1)
        .map(String::from)
        .collect::<Vec<String>>()
        .join("")
        .parse::<i32>()
        .unwrap();

    match command {
        'N' => way.1 += amount,
        'S' => way.1 -= amount,
        'E' => way.0 += amount,
        'W' => way.0 -= amount,
        'R' => way = rotate_waypoint(way, -amount),
        'L' => way = rotate_waypoint(way, amount),
        'F' => pos = (pos.0 + (way.0 * amount), pos.1 + (way.1 * amount)),
        _ => {}
    };

    (pos, way)
}

fn run_instructions_waypoint(instructions: &str, start: (i32, i32), waypoint: (i32, i32)) -> i32 {
    let mut pos = start;
    let mut way = waypoint;

    for instruction in instructions.split('\n') {
        let (new_pos, new_way) = handle_instruction_waypoint(instruction, pos, way);
        pos = new_pos;
        way = new_way;
    }

    pos.0.abs() + pos.1.abs()
}

fn main() {
    let input = fs::read_to_string("./day-12/input.txt").unwrap();

    println!("Part 1: {}", run_instructions(&input, (0, 0), 90));

    println!(
        "Part 2: {}",
        run_instructions_waypoint(&input, (0, 0), (10, 1))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_1() {
        let sample = "F10
N3
F7
R90
F11";
        assert_eq!(run_instructions(sample, (0, 0), 90), 25);
    }

    #[test]
    fn test_sample_2() {
        let sample = "F10
N3
F7
R90
F11";
        assert_eq!(run_instructions_waypoint(sample, (0, 0), (10, 1)), 286);
    }
}
