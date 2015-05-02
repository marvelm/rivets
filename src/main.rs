extern crate html5ever;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;
use std::default::Default;

use html5ever::sink::rcdom::RcDom;
use html5ever::{parse, one_input};

fn main() {
    let mut file = BufReader::new(File::open("ashdjashd").unwrap());
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    let dom: RcDom = parse(one_input(source.clone()), Default::default());
}
