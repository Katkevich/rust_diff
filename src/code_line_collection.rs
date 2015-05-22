use std::io::{ BufRead, BufReader, Lines };
use std::fs::{ File };
use std::cmp;
use std::borrow::Borrow;
use std::ops::*;
use std::iter::*;
use line_diff::{ LineDiff };
use diff_type::{ Diff };
use code_line::{ CodeLine };

pub struct CodeLineCollection {
    pub items: Vec<CodeLine>
}
