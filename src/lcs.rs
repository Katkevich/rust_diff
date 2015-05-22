use std::io::{ BufRead, BufReader, Lines };
use std::fs::{ File };
use std::cmp;
use std::borrow::Borrow;
use std::ops::*;
use std::iter::*;
use line_diff::{ LineDiff };
use diff_type::{ Diff };

pub struct LCS;

#[derive(Clone)]
pub struct LCSCell {
    pub lcs: i32,
    pub levenstein: i32,
}

impl LCSCell {
    pub fn new() -> LCSCell {
        LCSCell {
            lcs: 0,
            levenstein: 0,
        }
    }
}


impl LCS {
    pub fn levenstein2<T: PartialEq>(left: &Vec<T>, right: &Vec<T>) {
        let mut start = 0;
        let mut left_end = left.len();
        let mut right_end = right.len();

        while start < left_end && start < right_end && left[start] == right[start] {
            start = start + 1;
        }

        while start < left_end && start < right_end && left[left_end - 1] == right[right_end - 1] {
            left_end = left_end - 1;
            right_end = right_end - 1;
        }

        let matrix_rows = left_end - start + 1;
        let matrix_columns = right_end - start + 1;

        println!("Start: {}; LE: {}; RE: {}", start, left_end, right_end);

        let mut matrix = vec![vec![LCSCell::new(); matrix_columns]; matrix_rows];

        for row in 0..matrix_rows {
            matrix[row][0].levenstein = row as i32;
        }

        for col in 0..matrix_columns {
            matrix[0][col].levenstein = col as i32;
        }

        for row in start..left_end {
            for col in start..right_end {
                let i = row - start + 1;
                let j = col - start + 1;

                let (diff, lcs) = if left[row] == right[col] {
                    (0, matrix[i - 1][j - 1].lcs + 1)
                } else {
                    (1, cmp::max(matrix[i][j - 1].lcs, matrix[i - 1][j].lcs))
                };

                let levenstein = cmp::min(cmp::min(matrix[i - 1][j].levenstein + 1, matrix[i][j - 1].levenstein + 1), matrix[i - 1][j - 1].levenstein + diff);

                matrix[i][j].lcs = lcs;
                matrix[i][j].levenstein = levenstein;
            }
        }


        for r in 0..matrix_rows {
            for c in 0..matrix_columns {
                println!("M[{}][{}]: {}----{}", r, c, matrix[r][c].lcs, matrix[r][c].levenstein);
            }
            println!("");
        }
    }

    pub fn levenstein_distance<T: PartialEq>(left: &Vec<T>, right: &Vec<T>) -> i32 {
        let mut start = 0;
        let mut left_end = left.len();
        let mut right_end = right.len();

        while start < left_end && start < right_end && left[start] == right[start] {
            start = start + 1;
        }

        while start < left_end && start < right_end && left[left_end - 1] == right[right_end - 1] {
            left_end = left_end - 1;
            right_end = right_end - 1;
        }

        let matrix_rows = left_end - start + 1;
        let matrix_columns = right_end - start + 1;

        let mut matrix = vec![vec![0; matrix_columns]; matrix_rows];

        for row in 0..matrix_rows {
            matrix[row][0] = row as i32;
        }

        for col in 0..matrix_columns {
            matrix[0][col] = col as i32;
        }

        for row in start..left_end {
            for col in start..right_end {
                let i = row - start + 1;
                let j = col - start + 1;

                let diff = if left[row] == right[col] { 0 } else { 1 };
                matrix[i][j] = cmp::min(cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1), matrix[i - 1][j - 1] + diff);
            }
        }

        matrix[matrix_rows - 1][matrix_columns - 1]
    }

    pub fn get_lcs<T: PartialEq>(left: &Vec<T>, right: &Vec<T>) -> Vec<(usize, usize)> {
        let mut start = 0;
        let mut left_end = left.len();
        let mut right_end = right.len();

        while start < left_end && start < right_end && left[start] == right[start] {
            start = start + 1;
        }

        while start < left_end && start < right_end && left[left_end - 1] == right[right_end - 1] {
            left_end = left_end - 1;
            right_end = right_end - 1;
        }

        let matrix_rows = left_end - start + 1;
        let matrix_columns = right_end - start + 1;

        let mut path_matrix = vec![vec![0; matrix_columns]; matrix_rows];

        for row in start..left_end {
            for col in start..right_end {
                let i = row - start + 1;
                let j = col - start + 1;

                if left[row] == right[col] {
                    path_matrix[i][j] = path_matrix[i - 1][j - 1] + 1;
                } else {
                    path_matrix[i][j] = cmp::max(path_matrix[i][j - 1], path_matrix[i - 1][j]);
                }
            }
        }

        for r in 0..matrix_rows {
            for c in 0..matrix_columns {
                println!("M[{}][{}]: {}", r, c, path_matrix[r][c]);
            }
        }

        let lcs_pairs = LCS::select_lcs(&path_matrix, start, left_end, right_end, left.len(), right.len());

        lcs_pairs
    }

    fn select_lcs(path_matrix: &Vec<Vec<i32>>, start: usize, left_end: usize, right_end: usize, left_len: usize, right_len: usize) -> Vec<(usize, usize)> {
        let matrix_rows = left_end - start + 1;
        let matrix_columns = right_end - start + 1;

        let mut row = matrix_rows - 1;
        let mut col = matrix_columns - 1;
        let mut top_left_switch = false;

        let mut idxs = Vec::<(_, _)>::new();

        let skipped_suffix = (left_end..left_len).rev().zip((right_end..right_len).rev());
        for tuple in skipped_suffix {
            idxs.push(tuple);
        }

        loop {
            if path_matrix[row][col] == 0 {
                break;
            } else if path_matrix[row - 1][col] == path_matrix[row][col] {
                row = row - 1;
                continue;
            } else if path_matrix[row][col - 1] == path_matrix[row][col] {
                col = col - 1;
                continue;
            }

            idxs.push((start + row - 1, start + col - 1));
            row = row - 1;
            col = col - 1;
        }

        let skipped_prefix = (0..start).rev().map(|x| (x, x));
        for tuple in skipped_prefix {
            idxs.push(tuple);
        }

        idxs
    }
}
