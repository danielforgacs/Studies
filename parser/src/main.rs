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
            '0'..='9' => {
                it.next();
                let n = get_number(c, &mut it);
            },
            '+' | '-' => {
                result.push(LexItem::Op(c));
                it.next();
            },
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(LexItem::Paren(c));
                it.next();
            },
            ' ' => {
                it.next();
            },
            _ => {
                return Err(format!("unexpected char: {}", c));
            },
        }
    };
    Ok(result)
}

fn get_number<T: Iterator<Item = char>>(c: char, it: &mut std::iter::Peekable<T>) -> u64 {
    let mut number = c.to_string().parse::<u64>().expect("Not a digit.");
    while let Some(Ok(digit)) = it.peek().map(|s| s.to_string().parse::<u64>()) {
        number = number * 10 + digit;
        it.next();
    }
    number
}

fn main() {
    println!("Hello, world!");
}
