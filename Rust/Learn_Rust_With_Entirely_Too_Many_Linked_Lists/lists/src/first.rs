pub enum List {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: List,
}
