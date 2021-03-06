//use std::env;
extern crate regex;
 //#[feature(collections)]
// extern crate collections;

mod code_line;
mod line_diff;
mod diff_type;
mod diff_engine;
mod lcs;
mod slice_ext;

use std::ops::*;
use std::fs::{ File };
use std::io::{ BufReader };
use lcs::{ LCS, Changes };
use code_line::{ CodeLine, Line };
use diff_type::Diff;
use diff_engine::*;


fn main() {
    let source = open_file("D:\\111.txt");
    let target = open_file("D:\\222.txt");

    let source_lines = CodeLine::parse(source);
    let target_lines = CodeLine::parse(target);

    let mut engine = DiffEngine::new(source_lines, target_lines);
    engine.make_diff();
}

fn open_file(file_path: &str) -> BufReader<File> {
    //let args: Vec<_> = env::args().collect();

    let result = File::open(file_path);
    let file = match result {
            Err(err) => panic!("Can't open file {}", err),
            Ok(val) =>  val
        };
    let reader = BufReader::new(file);

    reader
}
