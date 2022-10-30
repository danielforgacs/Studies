enum GrammarItem {
    Product,
    Sum,
    Number(u64),
    Paren,
}

enum LexItem {
    Paren(char),
    Op(char),
    Num(u64),
}

struct ParseNode {
    children: Vec<ParseNode>,
    entry: GrammarItem,
}

impl ParseNode {
    fn new() -> Self {
        Self {
            children: Vec::new(),
            entry: GrammarItem::Paren,
        }
    }
}

fn lex(input: String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {},
            '+' | '-' => {},
            '(' | ')' | '[' | ']' | '{' | '}' => {},
            ' ' => {},
            _ => {},
        }
    };
    Ok(result)
}

fn main() {
    println!("Hello, world!");
}
