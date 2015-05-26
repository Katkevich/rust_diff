use std::io::{ BufRead, BufReader, Lines };
use std::fs::{ File };
use std::cmp;
use std::borrow::Borrow;
use std::ops::*;
use std::iter::*;
use line_diff::{ LineDiff };
use diff_type::{ Diff };
use regex::Regex;
use slice_ext::SliceExt;

pub trait Line {
    fn split<'a>(&'a self) -> Vec<&'a str>;

    fn set_diff(&mut self, diff: Diff);
}

pub struct CodeLine {
    pub code: String,
    pub line_number: i32,
    diff: Diff
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

impl Line for CodeLine {

    fn split<'a>(&'a self) -> Vec<&'a str> {
        let word_regex = Regex::new(r"\b\w+\b").unwrap();
        let words_iter = word_regex.find_iter(self.code.as_ref());
        let special_symbol_regex = Regex::new(r"\W").unwrap();
        let special_symbols_iter = special_symbol_regex.find_iter(self.code.as_ref());

        let mut previous_word_last_idx = 0;
        let line_last_idx = self.code.len();

        let mut result = Vec::<_>::new();

        for (from, to) in words_iter {
            if from - previous_word_last_idx > 0 {
                for char_idx in previous_word_last_idx..from {
                    let char_slice = self.code.slice(char_idx, char_idx + 1);
                    result.push(char_slice);
                }
            }

            let word_slice = self.code.slice(from, to);
            result.push(word_slice);

            previous_word_last_idx = to;
        }

        result
    }

    fn set_diff(&mut self, diff: Diff) {
        self.diff = diff;
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
