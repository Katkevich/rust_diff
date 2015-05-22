//use std::env;
mod code_line;
mod line_diff;
mod diff_type;
mod lcs;

use std::fs::{ File };
use std::io::{ BufReader };
use lcs::LCS;
use code_line::CodeLine;
use diff_type::Diff;

fn main() {
    let source = open_file("D:\\111.txt");
    let target = open_file("D:\\222.txt");

    let mut source_lines = CodeLine::parse(source);
    let mut target_lines = CodeLine::parse(target);

    let lcs_idxs = LCS::get_lcs(&source_lines, &target_lines);
    LCS::levenstein2(&source_lines, &target_lines);

    // for tuple in lcs_idxs {
    //     let (s, t) = tuple;
    //     source_lines[s].diff = Diff::NoChanges;
    //     target_lines[t].diff = Diff::NoChanges;
    // }





    // println!("Len: {}", lcs_idxs.len());
    // for tuple in lcs_idxs {
    //     let (a, b) = tuple;
    //     println!("{}-{}", a, b);
    // }
    //
    for line in source_lines {
        println!("Content S: {}, line_no: {}", line.line_number, line.code);
    }

    for line in target_lines {
        println!("Content T: {}, line_no: {}", line.line_number, line.code);
    }
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
