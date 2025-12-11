
use crate::vm::VM;

pub(crate) mod scanner;
pub(crate) mod token;
pub(crate) mod opcode;
pub(crate) mod chunk;
pub(crate) mod compiler;
pub(crate) mod value;
pub(crate) mod parse;
pub(crate) mod vm;

fn main() {
    let source = r#"
print 1 + 1
print 8*8
print 7*(5+2)/4*(100+(4*2))
7+7"#;
    let mut vm = VM::new();
    vm.interpret(source);        
}
