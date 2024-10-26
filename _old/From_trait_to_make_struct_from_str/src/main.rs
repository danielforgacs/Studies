struct Something {
    description: String,
}

impl From<&str> for Something {
    fn from(text: &str) -> Self {
        Something { description: text.to_string() }
    }
}

fn main() {
    let something = Something::from("hey, works");
    assert_eq!(something.description, "hey, works");
}
