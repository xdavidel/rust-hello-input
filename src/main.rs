use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();

    print!("Enter your name: ");
    io::stdout().flush().unwrap_or_default();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let (first_letter, rest) = input.split_at(1);

            println!("Hello {}{}", first_letter.to_uppercase(), rest);
        }
        Err(e) => {
            println!("Oops... somthing went wrong: {}.", e);
        }
    }
}
