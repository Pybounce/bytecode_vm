use std::env;

use crate::scanner::Scanner;

pub(crate) mod scanner;
pub(crate) mod token;
pub(crate) mod opcode;
pub(crate) mod chunk;
pub(crate) mod compiler;
pub(crate) mod value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source = "var x = 1 + 1";
    let mut scanner = Scanner::new(&source);
    scanner.scan_token();
}
