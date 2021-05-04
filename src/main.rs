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

/*
have a config file located at ~/.passstore/.config
* passmanager init <name> - create a new password store
                          - create a pgp key to encrypt the password file store with
                          - write the file to ~/.passwordstore/stores/<name>
  passmanager - ls of all password stores

  passmanager <name of store> --create --genpass --name <name of password>
  passmanager <name of store> --modify
  passmanager <name of store> --list

       password record:
            id:
            encrpytion:
            secret:
            access-time:
            last-modified:
            user-defined fields:
                - secret question

 */


///CLI menu for the program. Lists the options the user has, and takes user input.
fn menu() {
    let mut again = true; //loop control
    let mut buffer = String::new(); //store user input
    let mut selection: u32;
    while again {
        println!("\n*PASSWORD MANAGER CLI*\n");
        println!("1. generate new password");
        println!("9. exit\n");
        print!("Selection: ");
        io::stdout().flush().unwrap();

        //read user input
        io::stdin()
            .read_line(&mut buffer)
            .expect("could not read input");

        //parse user input to u32
        selection = buffer
            .trim()
            .parse()
            .expect("invalid user input, expecting integer");
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
