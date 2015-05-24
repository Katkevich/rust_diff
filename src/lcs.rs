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

pub struct Changes {
    pub left: Option<(usize, usize)>,
    pub right: Option<(usize, usize)>,
}

impl LCS {

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

    pub fn get_lcs_inv<T: PartialEq>(left: &Vec<T>, right: &Vec<T>) -> Vec<Changes> {
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

        for row in start..left_end {
            for col in start..right_end {
                let i = row - start + 1;
                let j = col - start + 1;

                if left[row] == right[col] {
                    matrix[i][j] = matrix[i - 1][j - 1] + 1;
                } else {
                    matrix[i][j] = cmp::max(matrix[i][j - 1], matrix[i - 1][j]);
                }
            }
        }

        let lcs_pairs = LCS::select_lcs_inv(&matrix, start, left_end, right_end, left.len(), right.len());

        lcs_pairs
    }

    fn select_lcs_inv(matrix: &Vec<Vec<i32>>, start: usize, left_end: usize, right_end: usize, left_len: usize, right_len: usize) ->
        Vec<Changes> {

        let mut left_from = left_end - start;
        let mut right_from = right_end - start;
        let mut left_to = left_from;
        let mut right_to = right_from;

        // for r in matrix {
        //     for c in r {
        //         print!("{} ", c);
        //     }
        //     println!("");
        // }

        let mut idxs = Vec::<_>::new();
        // println!("Start: {}; l_end: {}; r_end: {}", start, left_end, right_end);
        loop {
            if left_from > 0 && matrix[left_from - 1][right_from] == matrix[left_from][right_from] {
                left_from = left_from - 1;
                continue;
            } else if right_from > 0 && matrix[left_from][right_from - 1] == matrix[left_from][right_from] {
                right_from = right_from - 1;
                continue;
            }

            let left_interval = if left_to == left_from { None } else { Some((start + left_from, start + left_to - 1)) };
            let right_interval = if right_to == right_from { None } else { Some((start + right_from, start + right_to - 1)) };

            if left_interval != None || right_interval != None {
                idxs.push(Changes { left: left_interval, right: right_interval });
            }

            if left_from == 0 || right_from == 0 {
                break;
            }

            left_from = left_from - 1;
            right_from = right_from - 1;
            left_to = left_from;
            right_to = right_from;
        }

        idxs
    }
}
