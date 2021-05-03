use chrono::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io::{self, Write};

///generates a random alphanumeric string, with a length of the passed in integer
/// random character code from: (https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html)
fn genpass(length: u32) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect();
    rand_string
}

fn menu() {
    let mut again = true;
    let mut buffer = String::new();
    let mut _stdin = io::stdin();
    let mut selection: u32;
    while again {
        println!("\n*PASSWORD MANAGER CLI*\n");
        println!("1. generate new password");
        println!("9. exit\n");
        print!("Selection: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buffer)
            .expect("could not read input");
        selection = buffer.trim().parse().expect("invalid input");
        buffer.clear();

        let length: u32 = 20;
        match selection {
            1 => {
                let password = genpass(length);
                println!("\nGenerated password: {}", password);
                println!("Date generated: {}", Utc::now().date().naive_utc());
            }
            9 => {
                again = false;
            }
            _ => {
                println!("\n~Invalid input~")
            }
        }
    }
}

fn main() {
    menu();
}
