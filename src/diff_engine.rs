use lcs::{ LCS, Changes };
use code_line::{ CodeLine };
use std::io::{ BufRead, BufReader, Lines };
use std::fs::{ File };
use line_diff::{ LineDiff };
use diff_type::{ Diff };

pub struct DiffEngine {
    pub source_lines: Vec<CodeLine>,
    pub target_lines: Vec<CodeLine>,
}

impl DiffEngine {
    pub fn new(source: BufReader<File>, target: BufReader<File>) -> DiffEngine {
        let source_lines = DiffEngine::parse(source);
        let target_lines = DiffEngine::parse(target);

        DiffEngine {
            source_lines: source_lines,
            target_lines: target_lines,
        }
    }

    pub fn make_diff(&mut self) {
        let lcs_inv: Vec<Changes> = LCS::get_lcs_inv(&self.source_lines, &self.target_lines);

        for left_right_changes in &lcs_inv {
            let left_changes = left_right_changes.left;
            let right_changes = left_right_changes.right;

            if left_changes == Option::None {
                let (from, to) = right_changes.unwrap();
                for i in from..to + 1 {
                    self.target_lines[i].diff = Diff::Added;
                }
            }

            if right_changes == Option::None {
                let (from, to) = left_changes.unwrap();
                for i in from..to + 1 {
                    self.source_lines[i].diff = Diff::Removed;
                }
            }
        }
    }

    fn parse(reader: BufReader<File>) -> Vec<CodeLine> {
        let mut line_index = 0;
        let lines_iter = reader
            .lines()
            .map(|x| {
                line_index = line_index + 1;
                match x {
                    Err(err) => panic!("Invalid line {}", err),
                    Ok(val) => CodeLine::new(val, line_index)
                }
            });

        let lines = lines_iter.collect::<Vec<_>>();

        lines
    }


            // println!("Len: {}", lcs_inv.len());
            // for tuple in lcs_inv {
            //     let l = tuple.left;
            //     let r = tuple.right;
            //     match l {
            //         None => print!("none"),
            //         Some((f,t)) => print!("{},{}", f, t),
            //     }
            //     print!("---");
            //     match r {
            //         None => print!("none"),
            //         Some((f,t)) => print!("{},{}", f, t),
            //     }
            //     println!("");
            // }

}
