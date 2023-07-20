use clap::Command;

fn main() {
    let clargs = Command::new(std::env!("CARGO_PKG_NAME"))
    .get_matches();
    dbg!(&clargs);
}
