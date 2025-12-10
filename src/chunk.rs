use crate::value::Value;


pub struct Chunk {
    instructions: Vec<u8>,
    lines: Vec<u8>,
    constants: Vec<Value>
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
            lines: vec![],
            constants: vec![]
        }
    }
}