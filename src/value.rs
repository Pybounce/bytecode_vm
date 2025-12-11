use std::fmt::Display;

 #[derive(Clone, Copy, PartialEq, Debug)]
 pub enum Value {
    Bool(bool),
    Number(f64)
 }

 impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(v) => write!(f, "{}", v),
            Value::Number(v) => write!(f, "{}", v),
        }
    }
 }