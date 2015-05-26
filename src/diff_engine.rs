use lcs::{ LCS, Changes };
use code_line::{ CodeLine, Line };
use std::io::{ BufRead, BufReader, Lines };
use std::fs::{ File };
use line_diff::{ LineDiff };
use diff_type::{ Diff };

pub struct DiffEngine<T> {
    pub source_lines: Vec<T>,
    pub target_lines: Vec<T>,
}

impl<T> DiffEngine<T>
where T: Line + PartialEq {

    pub fn new(source_lines: Vec<T>, target_lines: Vec<T>) -> DiffEngine<T> {
        DiffEngine {
            source_lines: source_lines,
            target_lines: target_lines,
        }
    }

    pub fn make_diff(&mut self) {
        let lcs_inv: Vec<Changes> = LCS::get_lcs_inv(&self.source_lines, &self.target_lines);

        self.analize_lcs_inv(&lcs_inv);

        DiffEngine::<CodeLine>::output(&lcs_inv);
    }

    fn analize_lcs_inv(&mut self, lcs_inv: &Vec<Changes>) {
        for left_right_changes in lcs_inv {
            let left_changes = left_right_changes.left;
            let right_changes = left_right_changes.right;

            if left_changes == Option::None {
                let (from, to) = right_changes.unwrap();
                for i in from..to + 1 {
                    self.target_lines[i].set_diff(Diff::Added);
                }
            }

            if right_changes == Option::None {
                let (from, to) = left_changes.unwrap();
                for i in from..to + 1 {
                    self.source_lines[i].set_diff(Diff::Removed);
                }
            }

            let (left_from, left_to) = left_changes.unwrap();
            let (right_from, right_to) = right_changes.unwrap();
            let rows = left_to - left_from + 1;
            let cols = right_to - right_from + 1;

            let mut levenstein_dist_matrix = vec![vec![0; cols]; rows];

            for left_idx in left_from..left_to + 1 {
                for right_idx in right_from..right_to + 1 {
                    let i = left_idx - left_from;
                    let j = right_idx - right_from;

                    let left_words = self.source_lines[left_idx].split();
                    let right_words = self.target_lines[right_idx].split();

                    let dist = LCS::levenstein_distance(&left_words, &right_words);

                    levenstein_dist_matrix[i][j] = dist;
                }
            }

            for row in 0..rows {
                for col in 0..cols {

                }
            }
        }
    }

    fn calculate_min_diff(matrix: &Vec<Vec<i32>>, row_from: i32, col_from: i32, matrix_dimm: (i32, i32)) -> i32 {
        let (rows, cols) = matrix_dimm;
        let row = &matrix[row_from as usize];

        for i in col_from..cols {
            let min = DiffEngine::<CodeLine>::calculate_min_diff(matrix, row_from - 1, i, matrix_dimm);
        }

        1
    }

    fn output(lcs_inv: &Vec<Changes>) {
        println!("Len: {}", lcs_inv.len());
        for tuple in lcs_inv {
            let l = tuple.left;
            let r = tuple.right;
            match l {
                None => print!("none"),
                Some((f,t)) => print!("{},{}", f, t),
            }
            print!("---");
            match r {
                None => print!("none"),
                Some((f,t)) => print!("{},{}", f, t),
            }
            println!("");
        }
    }

    //
    // println!("");
    // for lw in &left_words {
    //     print!("{}_", lw);
    // }
    // println!("");
    // for rw in &right_words {
    //     print!("{}_", rw);
    // }
    // println!("");
}
