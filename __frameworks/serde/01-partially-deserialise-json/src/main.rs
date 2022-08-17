use serde::Deserialize;

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

}
