use chrono::prelude::*;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::fs::{create_dir_all, read_dir, write};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
//Uses the user passed store name to create a file in the .passmanager folder. The file will contain a generated password, date created and user entered name.

// Create function:
//  hash store name
//  validate store name: -> If no name, throw new error
//  get user and password from command line
//  encrypt secrets
//  append to file with new user/password

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

pub fn create(store_name: &str) {
    //Need a store name and then add secrets to that store
    //This can use the CLI Menu format that we had in the menu function
    //Can add option to allow auto generation of secrets or to allow a user to use their own
    let hdir = home::home_dir();
    match hdir {
        Some(path) => {
            let mut hdirfinal = path.display().to_string();
            hdirfinal.push_str("/.passmanager");

            if !Path::new(&hdirfinal).is_dir() {
                // Create dir if path doesn't exist
                println!("Base path does not exist!");
                let created = create_dir_all(&hdirfinal);
                match created {
                    Ok(()) => println!("New base path created"),
                    Err(e) => println!("Error creating new path: {}", e),
                }
            }

            let files = read_dir(&hdirfinal).unwrap();
            for file in files {
                if file.unwrap().file_name() == store_name {
                    println!("Store name already exists");
                    return;
                }
            }

            //store name does not already exist
            //creating path for new file
            let mut pathfilestring: String = "".to_owned();
            pathfilestring.push_str(&hdirfinal);
            pathfilestring.push('/');
            pathfilestring.push_str(store_name);
            pathfilestring.push_str(".txt");

            let mut passfile = PathBuf::new();
            passfile.push(pathfilestring);

            //get data for the store
            let file_data = file_data();

            //create string to store in file
            let mut text = "test";

            // text.push_str("id: ");
            // text.push_str(&file_data.id);
            // text.push_str("\n");
            // text.push_str("secret: ");
            // text.push_str(&file_data.pass);
            // text.push_str("\n");
            // text.push_str("date: ");
            // text.push_str(&file_data.date);
            // text.push_str("\n");

            let written = write(path, text);
            match written {
                Ok(()) => println!("Successfully written to file"),
                Err(e) => println!("Unable to write to file: {}", e),
            }
        }
        None => {
            println!("Impossible to get your home dir!");
            return;
        }
    }
}

struct FileData {
    id: String,
    pass: String,
    date: String,
}

fn file_data() -> FileData {
    let mut buffer = String::new();
    print!("\nEnter name associated with data: ");
    io::stdout().flush().unwrap();

    //read user input
    io::stdin()
        .read_line(&mut buffer)
        .expect("could not read input");

    buffer.trim();

    let name: String;
    name = buffer;

    let data = FileData {
        pass: genpass(20),
        id: name,
        date: Utc::now().date().naive_utc().to_string(),
    };
    return data;
}
