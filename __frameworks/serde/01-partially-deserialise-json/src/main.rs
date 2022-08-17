use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
struct J {
    a: u8,
    b: u8,
}

fn main() {
    let json = r#"[
        {
            "a": 1,
            "b": 1
        }
    ]"#;
    let data: Vec<J> = serde_json::from_str(&json).expect("SHIT");
    for item in data {
        println!("data.a: {}", item.a);
    }
}
