enum Link {
    Empty,
    More(Box<Node>),
}

struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: List,
}
