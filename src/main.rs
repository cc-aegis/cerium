pub mod lexer;
pub mod error;

fn main() {
    let code = "
// function
fn main() {
    let x: u16* = &[1, 2, 3, 4];
    io::print(x);
}

    ";
    let mut lexer = lexer::Lexer::new(code);
    while let Ok(Some(token)) = lexer.next() {
        println!("{:?}", token);
    }
}