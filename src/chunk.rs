
#[derive(Clone)]
pub enum OpCode {
    Return
}

pub struct Chunk {
    pub codes: Vec<OpCode>
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            codes: vec![]
        }
    }
    pub fn write(&mut self, op: OpCode) {
        self.codes.push(op);
    }
}