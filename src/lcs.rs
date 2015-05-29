use std::io::{ BufRead, BufReader, Lines };
use std::fs::{ File };
use std::cmp;
use std::borrow::Borrow;
use std::ops::*;
use std::iter::*;
use line_diff::{ LineDiff };
use diff_type::{ Diff };
use code_line::{ CodeLine, Line };

pub struct LCS;

pub struct Changes {
    pub source: Option<(usize, usize)>,
    pub target: Option<(usize, usize)>,
}

impl LCS {

    pub fn levenstein_distance<T: PartialEq>(source: &Vec<T>, target: &Vec<T>) -> i32 {
        let mut start = 0;
        let mut source_end = source.len();
        let mut target_end = target.len();

        while start < source_end && start < target_end && source[start] == target[start] {
            start = start + 1;
        }

        while start < source_end && start < target_end && source[source_end - 1] == target[target_end - 1] {
            source_end = source_end - 1;
            target_end = target_end - 1;
        }

        let matrix_rows = source_end - start + 1;
        let matrix_columns = target_end - start + 1;

        let mut matrix = vec![vec![0; matrix_columns]; matrix_rows];

        for row in 0..matrix_rows {
            matrix[row][0] = row as i32;
        }

        for col in 0..matrix_columns {
            matrix[0][col] = col as i32;
        }

        for row in start..source_end {
            for col in start..target_end {
                let i = row - start + 1;
                let j = col - start + 1;

                let diff = if source[row] == target[col] { 0 } else { 1 };
                matrix[i][j] = cmp::min(cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1), matrix[i - 1][j - 1] + diff);
            }
        }

        // for r in &matrix {
        //     for c in r {
        //         print!("{} ", c);
        //     }
        //     println!("");
        // }

        matrix[matrix_rows - 1][matrix_columns - 1]
    }

    pub fn get_lcs_inverted<T: PartialEq>(source: &Vec<T>, target: &Vec<T>) -> (Vec<Changes>, i32) {
        let mut start = 0;
        let mut source_end = source.len();
        let mut target_end = target.len();

        while start < source_end && start < target_end && source[start] == target[start] {
            start = start + 1;
        }

        while start < source_end && start < target_end && source[source_end - 1] == target[target_end - 1] {
            source_end = source_end - 1;
            target_end = target_end - 1;
        }

        let matrix_rows = source_end - start + 1;
        let matrix_columns = target_end - start + 1;

        let mut matrix = vec![vec![0; matrix_columns]; matrix_rows];

        for row in start..source_end {
            for col in start..target_end {
                let i = row - start + 1;
                let j = col - start + 1;

                if source[row] == target[col] {
                    matrix[i][j] = matrix[i - 1][j - 1] + 1;
                } else {
                    matrix[i][j] = cmp::max(matrix[i][j - 1], matrix[i - 1][j]);
                }
            }
        }

        let lcs_pairs = LCS::select_lcs_inverted(&matrix, start, source_end, target_end, source.len(), target.len());

        (lcs_pairs, matrix[matrix_rows - 1][matrix_columns - 1])
    }

    fn select_lcs_inverted(matrix: &Vec<Vec<i32>>, start: usize, source_end: usize, target_end: usize, source_len: usize, target_len: usize) -> Vec<Changes> {

        let mut source_from = source_end - start;
        let mut target_from = target_end - start;
        let mut source_to = source_from;
        let mut target_to = target_from;

        // for r in matrix {
        //     for c in r {
        //         print!("{} ", c);
        //     }
        //     println!("");
        // }

        let mut idxs = Vec::<_>::new();
        // println!("Start: {}; l_end: {}; r_end: {}", start, source_end, target_end);
        loop {
            if source_from > 0 && matrix[source_from - 1][target_from] == matrix[source_from][target_from] {
                source_from = source_from - 1;
                continue;
            } else if target_from > 0 && matrix[source_from][target_from - 1] == matrix[source_from][target_from] {
                target_from = target_from - 1;
                continue;
            }

            let source_interval = if source_to == source_from { None } else { Some((start + source_from, start + source_to - 1)) };
            let target_interval = if target_to == target_from { None } else { Some((start + target_from, start + target_to - 1)) };

            if source_interval != None || target_interval != None {
                idxs.push(Changes { source: source_interval, target: target_interval });
            }
            if source_from == 0 || target_from == 0 {
                break;
            }

            source_from = source_from - 1;
            target_from = target_from - 1;
            source_to = source_from;
            target_to = target_from;
        }

        idxs
    }
}
