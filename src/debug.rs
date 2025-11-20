use crate::chunk::Chunk;


pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.codes.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    let instruction = &chunk.codes[offset];
    return match instruction {
        crate::chunk::OpCode::Return => simple_instruction("RETURN", offset),
    };
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    print!("{}\n", name);
    return offset + 1;
}