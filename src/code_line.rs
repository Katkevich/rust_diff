use std::io::{ BufRead, BufReader, Lines };
use std::fs::{ File };
use std::cmp;
use std::borrow::Borrow;
use std::ops::*;
use std::iter::*;
use line_diff::{ LineDiff };
use diff_type::{ Diff };

pub struct CodeLine {
    pub code: String,
    pub line_number: i32,
    pub diff: Diff
}

impl CodeLine {
    pub fn new(code: String, line_number: i32) -> CodeLine {
        CodeLine {
            code: code,
            line_number: line_number,
            diff: Diff::NoChanges
        }
    }

    pub fn parse(reader: BufReader<File>) -> Vec<CodeLine> {
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
}

impl PartialEq for CodeLine {
    fn eq(&self, other: &CodeLine) -> bool {
        self.code == other.code
    }

    fn ne(&self, other: &CodeLine) -> bool {
        !self.eq(other)
    }
}
