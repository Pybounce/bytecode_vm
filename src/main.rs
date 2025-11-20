use std::env;

use crate::{chunk::{Chunk, OpCode}, debug::disassemble_chunk};


pub mod chunk;
pub mod debug;

fn main() {
    //let args: Vec<String> = env::args().collect();
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Return);

    disassemble_chunk(&chunk, "test chunk");
}
