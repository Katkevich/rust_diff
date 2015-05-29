use lcs::{ LCS, Changes };
use code_line::{ CodeLine, Line };
// use std::io::{ BufRead, BufReader, Lines };
// use std::fs::{ File };
// use line_diff::{ LineDiff };
use diff_type::{ Diff };
use std::ops::*;
// use std::iter::*;
use std::cmp;
use slice_ext::SliceExt;

pub struct DiffEngine<T> {
    pub source_lines: Vec<T>,
    pub target_lines: Vec<T>,
}

#[derive(Clone)]
struct ConflictMatrixCell {
    pub distance: i32,
    pub next_rows_distance_accum: i32
}

impl ConflictMatrixCell {
    fn new() -> ConflictMatrixCell {
        ConflictMatrixCell {
            distance: 0,
            next_rows_distance_accum: 0
        }
    }
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
        let (lcs_inv, lcs_distance) = LCS::get_lcs_inverted(&self.source_lines, &self.target_lines);

        self.analize_lcs_inverted(&lcs_inv);

        DiffEngine::<CodeLine>::output(&lcs_inv);
    }

    fn analize_lcs_inverted(&mut self, lcs_inv: &Vec<Changes>) {
        for changes in lcs_inv {
            let source_changes = changes.source;
            let target_changes = changes.target;

            if source_changes == Option::None {
                self.mark_target_as_added(target_changes.unwrap());
            }

            if target_changes == Option::None {
                self.mark_source_as_removed(source_changes.unwrap());
            }

            self.resolve_conflict(source_changes.unwrap(), target_changes.unwrap());
        }
    }

    fn mark_target_as_added(&mut self, interval: (usize, usize)) {
        let (from, to) = interval;
        for i in from..to + 1 {
            self.target_lines[i].set_diff(Diff::Added);
        }
    }

    fn mark_source_as_removed(&mut self, interval: (usize, usize)) {
        let (from, to) = interval;
        for i in from..to + 1 {
            self.source_lines[i].set_diff(Diff::Removed);
        }
    }

    fn resolve_conflict(&mut self, source_changes_interval: (usize, usize), target_changes_interval: (usize, usize)) {
        let (source_from, source_to) = source_changes_interval;
        let (target_from, target_to) = target_changes_interval;
        let rows = source_to - source_from + 1;
        let cols = target_to - target_from + 1;

        let mut conflict_matrix = vec![vec![ConflictMatrixCell::new(); cols]; rows];

        // println!("{}:{}-{}:{}", source_from, source_to, target_from, target_to);

        for source_idx in (source_from..source_to + 1).rev() {
            for target_idx in (target_from..target_to + 1).rev() {
                let i = source_idx - source_from;
                let j = target_idx - target_from;

                if j < source_idx - source_from {
                    break;
                }

                let source_words = self.source_lines[source_idx].split();
                let target_words = self.target_lines[target_idx].split();

                let dist = LCS::levenstein_distance(&source_words, &target_words);

                let next_row_min_dist_sum = self.next_row_min_distance_sum(&conflict_matrix, i + 1, j);

                conflict_matrix[i][j].distance = dist;
                conflict_matrix[i][j].next_rows_distance_accum = next_row_min_dist_sum;
            }
        }

        // for r in &conflict_matrix {
        //     for c in r {
        //         print!("{}/{} ", c.distance, c.next_rows_distance_accum);
        //     }
        //     println!("");
        // }
    }

    fn next_row_min_distance_sum(&self, matrix: &Vec<Vec<ConflictMatrixCell>>, row_idx: usize, col_from: usize) -> i32 {
        if row_idx >= matrix.len() {
            return 0;
        }

        let row = &matrix[row_idx];
        let row_slice = row.slice(col_from, row.len());

        let mut min = i32::max_value();

        for cell in row_slice {
            min = cmp::min(min, cell.distance + cell.next_rows_distance_accum);
        }

        min
    }

    fn output(lcs_inv: &Vec<Changes>) {
        println!("Len: {}", lcs_inv.len());
        for tuple in lcs_inv {
            let l = tuple.source;
            let r = tuple.target;
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
    // for lw in &source_words {
    //     print!("{}_", lw);
    // }
    // println!("");
    // for rw in &target_words {
    //     print!("{}_", rw);
    // }
    // println!("");
}
