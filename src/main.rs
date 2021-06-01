use chrono::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::{read_dir, create_dir_all, write};
use std::io::{self, Write};
use std::path::{PathBuf, Path};
use google_authenticator;
use google_authenticator::{GoogleAuthenticator, ErrorCorrectionLevel};
use rpassword::read_password;

mod init;

extern crate home;

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

/*
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
*/

fn get_stores() {
    let hdir = home::home_dir();
    match hdir {
        Some(path) => {
            let mut hdirfinal = path.display().to_string();
            hdirfinal.push_str("/.passmanager");
            let testfiles = read_dir(&hdirfinal);
            match testfiles {
                Ok(_v) => (),
                Err(_e) => {
                    println!("Error: the base path does not exist or the process lacks permissions to view the contents");
                    return;
                }
            }

            let files = read_dir(&hdirfinal).unwrap();

            //print names of all files in the base directory
            for file in files {
                println!("Filename: {:?}", file.unwrap().file_name())
            }
            //TODO: Crawl through password dir and print all store names
            //TODO: Maybe store names should be encrypted as well?
            //TODO: Decrypt store names and print to screen
        }
        None => {
            println!("Impossible to get your home dir!");
        }
    }
}

fn create(store_name: &str) {
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

            let passanddate = passanddate();
            let written = write(path, passanddate);
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

fn passanddate() -> String {
    let password = genpass(20);
    let date = Utc::now().date().naive_utc().to_string();
    let mut finalwrite = password.clone();
    finalwrite.push('\n');
    finalwrite.push_str(&date);
    return finalwrite;
}

fn authenticate(){
    todo!()
}

fn setup_mfa(auth: GoogleAuthenticator, secret: &str){
    // Set up authenticator
    //let auth = GoogleAuthenticator::new();
    println!(
        "{}",
        auth.qr_code_url(secret, "qr_code", "name", 200, 200, ErrorCorrectionLevel::High)
    );
}

fn check_secret(auth: GoogleAuthenticator){
    let secret = "I3VFM3JKMNDJCDH5BMBEEQAW6KJ6NOE4";
    let code = auth.get_code(secret, 0).unwrap();
    if auth.verify_code(secret, code.as_str(), 1, 0){
        println!("match!");
    }else{
        println!("false!")
    }
}

// Gets user password without revealing it on the command line
fn get_password(){
    print!("Password: ");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    println!("The password is: {}", password);
}

fn first_boot(){
    // Setup password for general operation of CLI

    // Encrypt that password

}

fn main() {
    // Get all command line args
    let args: Vec<String> = std::env::args().collect();

    //TODO: Create config file
    // Create master key and secret key
    //

    //TODO: Before we can give user access to stores and functions user must enter password
    // Password gives user a set of credentials to unlock stores
    // Creds will last a certain amount of time before a user must reauthenicate

    //TODO: Require password entry


    let error = init::set_global_password();
    match error {
        Ok(_) => println!("The password is:"),
        Err(e) => println!("error: {}", e),
    };

    let error = init::get_password();
    match error {
        Ok(_) => println!("The password is:"),
        Err(e) => println!("error: {}", e),
    };

    //get_password();

    // let auth = GoogleAuthenticator::new().copy();
    // let secret = "I3VFM3JKMNDJCDH5BMBEEQAW6KJ6NOE3";
    // setup(auth);
    // check_secret(auth);

    // List all password stores
    if args.len() == 1 {
        println!("Getting all password stores!");
        get_stores();
        return;
    }

    // Parse all other args
    match args[1].as_str() {
        "init" => {
            // TODO: Catch the panic here and just print message
            let store_name = args
                .get(2)
                .expect("Did not get store name for option 'Init'");
            println!("Init new password store: {}", store_name);
            //init::init(store_name);
        }
        "create" => {
            println!("Created new password");
            create(args[2].as_str());
        }
        _ => {
            println!("Unknown arg: {}", args[1])
        }
    }
}
