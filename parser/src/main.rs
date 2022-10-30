#[derive(Debug)]
enum GrammarItem {
    Product,
    Sum,
    Number(u64),
    Paren,
}

#[derive(Debug)]
enum LexItem {
    Paren(char),
    Op(char),
    Num(u64),
}

#[derive(Debug)]
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

fn parse(input: String) -> Result<ParseNode, String> {
    let tokens = lex(input)?;
    parse_expr(&tokens, 0).and_then(|(n, i)| if i == tokens.len() {
            Ok(n)
        } else {
            Err("shit".to_string())
    })
}

fn parse_expr(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (node_summand, next_pos) = parse_summand(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('+')) => {
            let mut sum = ParseNode::new();
            sum.entry = GrammarItem::Sum;
            sum.children.push(node_summand);
            let (rhs, i) = parse_expr(tokens, next_pos + 1)?;
            sum.children.push(rhs);
            Ok((sum, i))
        }
        _ => {
            Ok((node_summand, next_pos))
        }
    }
}

fn parse_summand(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (node_term, next_pos) = parse_term(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('*')) => {
            let mut sum = ParseNode::new();
            sum.entry = GrammarItem::Product;
            sum.children.push(node_term);
            let (rhs, i) = parse_expr(tokens, next_pos + 1)?;
            sum.children.push(rhs);
            Ok((sum, i))
        }
        _ => {
            Ok((node_term, next_pos))
        }
    }

}

fn main() {
    let result = lex("() [ {} ] 87567 + [ ] (-+-)".to_string()).unwrap();
    dbg!(&result);
}
