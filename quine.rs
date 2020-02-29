/// Rust Quine

const CODE : &str = include_str!(file!());
fn main() {
    print!("{}", CODE);
}