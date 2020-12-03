use std::fs;

fn row_index_is_tree(row: &str, index: usize) -> bool {
    row.chars().nth(index % row.len()).unwrap() == '#'
}

fn count_grid_trees(grid: Vec<String>, start_x: usize, x_diff: usize, y_diff: usize) -> usize {
    grid.iter()
        .enumerate()
        .filter(|(idx, row)| {
            idx % y_diff == 0 && row_index_is_tree(row, start_x + (x_diff * (idx / y_diff)))
        })
        .count()
}

fn main() {
    let input = fs::read_to_string("./day-03/input.txt").unwrap();
    let grid: Vec<String> = input.split('\n').map(String::from).collect();

    println!("Part 1: {}", count_grid_trees(grid.clone(), 0, 3, 1));

    let slope_vec = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let trees_product: usize = slope_vec
        .iter()
        .map(|(x, y)| count_grid_trees(grid.clone(), 0, *x, *y))
        .product();

    println!("Part 2: {}", trees_product);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_row_index_is_tree() {
        assert!(!row_index_is_tree("..##.......", 0));
        assert!(row_index_is_tree("..##.......", 2));
    }

    #[test]
    fn test_count_grid_trees_sample_1() {
        let grid = "..##.......
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
        let grid_vec = grid.split('\n').map(String::from).collect();
        assert_eq!(count_grid_trees(grid_vec, 0, 3, 1), 7);
    }

    #[test]
    fn test_count_grid_trees_sample_2() {
        let grid = "..##.......
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
        let grid_vec: Vec<String> = grid.split('\n').map(String::from).collect();
        let input_vec = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        let tree_counts: Vec<usize> = input_vec
            .iter()
            .map(|(x, y)| count_grid_trees(grid_vec.clone(), 0, *x, *y))
            .collect();
        assert_eq!(tree_counts[0], 2);
        assert_eq!(tree_counts[1], 7);
        assert_eq!(tree_counts[2], 3);
        assert_eq!(tree_counts[3], 4);
        assert_eq!(tree_counts[4], 2);
        assert_eq!(tree_counts.iter().product::<usize>(), 336);
    }
}
