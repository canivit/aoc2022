use array2d::{Array2D, Error};

pub fn process_part1(input: &str) -> Result<usize, Error> {
    let grid = build_grid(input)?;
    Ok(count_visible(&grid))
}

pub fn process_part2(input: &str) -> Result<usize, Error> {
    let grid = build_grid(input)?;
    Ok(highest_scenic_score(&grid))
}

fn build_grid(input: &str) -> Result<Array2D<u32>, Error> {
    let rows: Vec<_> = input.lines().map(|line| build_row(line)).collect();
    Array2D::from_rows(&rows)
}

fn build_row(line: &str) -> Vec<u32> {
    line.chars()
        .filter_map(|c| c.to_string().parse::<u32>().ok())
        .collect()
}

fn count_visible(grid: &Array2D<u32>) -> usize {
    grid.indices_row_major()
        .filter(|pair| is_visible(grid, pair.0, pair.1))
        .count()
}

fn highest_scenic_score(grid: &Array2D<u32>) -> usize {
    grid.indices_row_major()
        .map(|pair| scenic_score(grid, pair.0, pair.1))
        .max()
        .unwrap_or(0)
}

fn is_visible(grid: &Array2D<u32>, row_idx: usize, column_idx: usize) -> bool {
    match (grid.row_iter(row_idx), grid.column_iter(column_idx)) {
        (Ok(row_iter), Ok(column_iter)) => {
            let row: Vec<u32> = row_iter.copied().collect();
            let column: Vec<u32> = column_iter.copied().collect();
            is_visible_from_top(&column, row_idx)
                || is_visible_from_bottom(&column, row_idx)
                || is_visible_from_left(&row, column_idx)
                || is_visible_from_right(&row, column_idx)
        }
        _ => false,
    }
}

fn scenic_score(grid: &Array2D<u32>, row_idx: usize, column_idx: usize) -> usize {
    match (grid.row_iter(row_idx), grid.column_iter(column_idx)) {
        (Ok(row_iter), Ok(column_iter)) => {
            let row: Vec<u32> = row_iter.copied().collect();
            let column: Vec<u32> = column_iter.copied().collect();
            scenic_score_top_left(&column, row_idx)
                * scenic_score_bottom_right(&column, row_idx)
                * scenic_score_top_left(&row, column_idx)
                * scenic_score_bottom_right(&row, column_idx)
        }
        _ => 0,
    }
}

fn is_visible_from_top(column: &[u32], row_idx: usize) -> bool {
    is_greater_than_rest(column[row_idx], &column[0..row_idx])
}

fn is_visible_from_bottom(column: &[u32], row_idx: usize) -> bool {
    is_greater_than_rest(column[row_idx], &column[row_idx + 1..column.len()])
}

fn is_visible_from_left(row: &[u32], column_idx: usize) -> bool {
    is_greater_than_rest(row[column_idx], &row[0..column_idx])
}

fn is_visible_from_right(row: &[u32], column_idx: usize) -> bool {
    is_greater_than_rest(row[column_idx], &row[column_idx + 1..row.len()])
}

fn is_greater_than_rest(value: u32, rest: &[u32]) -> bool {
    rest.iter().find(|v| **v >= value).is_none()
}

fn scenic_score_top_left(block: &[u32], idx: usize) -> usize {
    block[0..idx]
        .iter()
        .rev()
        .enumerate()
        .find(|pair| *pair.1 >= block[idx])
        .map_or(idx, |pair| pair.0 + 1)
}

fn scenic_score_bottom_right(block: &[u32], idx: usize) -> usize {
    block[idx + 1..block.len()]
        .iter()
        .enumerate()
        .find(|pair| *pair.1 >= block[idx])
        .map_or(block.len() - 1 - idx, |pair| pair.0 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_visible() {
        let grid: Array2D<u32> = Array2D::from_rows(&vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ])
        .unwrap();

        assert_eq!(21, count_visible(&grid))
    }

    #[test]
    fn test_highest_scenic_score() {
        let grid: Array2D<u32> = Array2D::from_rows(&vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ])
        .unwrap();

        assert_eq!(8, highest_scenic_score(&grid))
    }

    #[test]
    fn test_is_visible() {
        let grid: Array2D<u32> = Array2D::from_rows(&vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ])
        .unwrap();

        assert!(is_visible(&grid, 2, 3));
        assert!(is_visible(&grid, 1, 1));
        assert!(is_visible(&grid, 4, 3));

        assert!(!is_visible(&grid, 1, 3));
        assert!(!is_visible(&grid, 2, 2));
        assert!(!is_visible(&grid, 3, 1));
        assert!(!is_visible(&grid, 3, 3));
    }

    #[test]
    fn test_scenic_score() {
        let grid: Array2D<u32> = Array2D::from_rows(&vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ])
        .unwrap();

        assert_eq!(8, scenic_score(&grid, 3, 2));
        assert_eq!(4, scenic_score(&grid, 1, 2))
    }

    #[test]
    fn test_is_visible_from_top() {
        let column = &vec![7, 1, 3, 4, 9];
        assert!(is_visible_from_top(column, 0));
        assert!(is_visible_from_top(column, 4));
        assert!(!is_visible_from_top(column, 1));
    }

    #[test]
    fn test_is_visible_from_left() {
        let row = &vec![2, 5, 5, 1, 2];
        assert!(is_visible_from_left(row, 0));
        assert!(is_visible_from_left(row, 1));
        assert!(!is_visible_from_left(row, 2));
        assert!(!is_visible_from_left(row, 3));
    }

    #[test]
    fn test_is_visible_from_right() {
        let row = &vec![2, 5, 5, 1, 2];
        assert!(is_visible_from_right(row, 4));
        assert!(!is_visible_from_right(row, 3));
        assert!(is_visible_from_right(row, 2));
        assert!(!is_visible_from_right(row, 1));
    }

    #[test]
    fn test_scenic_score_top_left() {
        // top
        let column = &vec![3, 5, 3, 5, 3];
        assert_eq!(0, scenic_score_top_left(column, 0));
        assert_eq!(1, scenic_score_top_left(column, 1));
        assert_eq!(1, scenic_score_top_left(column, 2));
        assert_eq!(2, scenic_score_top_left(column, 3));
        assert_eq!(1, scenic_score_top_left(column, 4));

        // left
        let row = &vec![2, 5, 5, 1, 2];
        assert_eq!(0, scenic_score_top_left(row, 0));
        assert_eq!(1, scenic_score_top_left(row, 1));
        assert_eq!(1, scenic_score_top_left(row, 2));
        assert_eq!(1, scenic_score_top_left(row, 3));
        assert_eq!(2, scenic_score_top_left(row, 4));
    }

    #[test]
    fn test_scenic_score_bottom_right() {
        // bottom
        let column = &vec![3, 5, 3, 5, 3];
        assert_eq!(1, scenic_score_bottom_right(column, 0));
        assert_eq!(2, scenic_score_bottom_right(column, 1));
        assert_eq!(1, scenic_score_bottom_right(column, 2));
        assert_eq!(1, scenic_score_bottom_right(column, 3));
        assert_eq!(0, scenic_score_bottom_right(column, 4));

        // right
        let row = &vec![2, 5, 5, 1, 2];
        assert_eq!(1, scenic_score_bottom_right(row, 0));
        assert_eq!(1, scenic_score_bottom_right(row, 1));
        assert_eq!(2, scenic_score_bottom_right(row, 2));
        assert_eq!(1, scenic_score_bottom_right(row, 3));
        assert_eq!(0, scenic_score_bottom_right(row, 4));
    }
}
