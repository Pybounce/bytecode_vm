use crate::{chunk::Chunk, compiler::Compiler, opcode::OpCode, value::Value};


pub struct VM {
    pub pc: usize,
    pub stack: Vec<Value>,
    pub chunk: Chunk
}

impl VM {
    pub fn new() -> Self {
        Self {
            pc: 0,
            stack: Vec::with_capacity(32),
            chunk: Chunk::new_terminated(),
        }
    }

    pub fn interpret(&mut self, source: &str) -> bool {
        let compiler = Compiler::new(source);
        if let Some(chunk) = compiler.compile() {
            self.chunk = chunk;
            self.pc = 0;
            return self.run();
        }

        return false;
    }

    fn run(&mut self) -> bool {
        loop {
            let operation = OpCode::try_from(self.read_byte());
            if operation.is_err() { 
                self.runtime_error("Failed to convert byte to opcode");
                return false;
            }
            match operation.unwrap() {
                OpCode::Constant => {
                    let val = self.read_constant();
                    self.stack.push(val);
                },
                OpCode::Print => {
                    println!("{}", self.stack.pop().unwrap())
                },
                OpCode::Pop => { self.stack.pop(); },
                OpCode::Equal => todo!(),
                OpCode::Not => todo!(),
                OpCode::Greater => { if !self.binary_number_op(|a, b| Value::Bool(a > b)) { return false; } },
                OpCode::Less => { if !self.binary_number_op(|a, b| Value::Bool(a < b)) { return false; } },
                OpCode::Add => { if !self.binary_number_op(|a, b| Value::Number(a + b)) { return false; } },
                OpCode::Subtract => { if !self.binary_number_op(|a, b| Value::Number(a - b)) { return false; } },
                OpCode::Multiply => { if !self.binary_number_op(|a, b| Value::Number(a * b)) { return false; } },
                OpCode::Divide => { if !self.binary_number_op(|a, b| Value::Number(a / b)) { return false; } },
                OpCode::Negate => {
                    let val = self.stack.pop().unwrap();
                    if let Value::Number(n) = val {
                        self.stack.push(Value::Number(-n));
                    } else { 
                        self.runtime_error("Negate operand must be number."); 
                        return false;
                    }
                },
                OpCode::Return => return true
            }
        }
    }
}


// Helpers
impl VM {
    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.bytes[self.pc];
        self.pc += 1;
        return byte;
    }
    fn read_constant(&mut self) -> Value {
        let index = self.read_byte() as usize;
        return self.chunk.constants[index];
    }
    fn runtime_error(&mut self, message: &'static str) {
        todo!()
    }
    fn binary_number_op<T>(&mut self, apply: T) -> bool where T: Fn(f64, f64) -> Value {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        match (a, b) {
            (Value::Number(num_a), Value::Number(num_b)) => {
                self.stack.push(apply(num_a, num_b));
                return true;
            },
            _ => { 
                self.runtime_error("Add operands must be numbers");
                return false;
             }
        }
    } 
}