use crate::lexer::Lexer;

mod lexer;
mod error;

fn main() {
    let code = "
fn main() {
    let utof: &fn(u16) -> f16 = &fn(int: u16) -> f16 {
        int as f16
    };
}
    ";
    let mut lexer = Lexer::new(code);
    while let Some(token) = lexer.next() {
        println!("{token:?}");
    }
}
