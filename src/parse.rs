
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ParseRule {
    pub prefix: ParseFn,
    pub infix: ParseFn,
    pub precedence: ParsePrecedence,
}

impl ParseRule {
    pub fn new(prefix: ParseFn, infix: ParseFn, precedence: ParsePrecedence) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ParsePrecedence {
    None,           // Low Precedence
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,        // High Precedence
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ParseFn {
    None,   // neater than using option everywhere, sue me. (don't sue me)
    Number,
    Binary,
    Grouping,
    Call,
    Unary,
    Variable,
    String,
    Literal,
    And,
    Or
}
