use clap::{App, Arg};

mod operations {
    pub mod create;
    pub mod delete;
    pub mod get;
    pub mod init;
    pub mod modify;
}

mod models {
    pub mod data_model;
}

mod generic {
    pub mod common;
    pub mod errors;
    pub mod encryption;
}

fn main() {
    // Get all command line args
    let matches = App::new("Password Manager")
        .version("1.0")
        .author("Haohan Jiang <jiang4.pdx.edu>, Taraq Jallad <tajallad@pdx.edu>")
        .about(
            "Usage: \n\
        To initialize a new store: password_manager <NAME> init\n\
        To delete a secret from the store: password_manager <NAME> delete\n\
        To add a secret to the store: password_manager <NAME> create\n",
        )
        .arg(
            Arg::new("store_name")
                .short('s')
                .long("store_name")
                .value_name("NAME")
                .about("Store name for operation<op>.")
                .takes_value(true),
        )
        .arg(
            Arg::new("op")
                .long("op")
                .value_name("OPERATION")
                .about(
                    "Action to perform on the password store. Available: init, create, get, delete",
                )
                .takes_value(true),
        )
        .arg(
            Arg::new("entry_name")
                .short('e')
                .long("entry_name")
                .about("Entry name for operation<op>.")
                .takes_value(true),
        )
        .get_matches();

    let store_name = matches.value_of("store_name").unwrap();
    if let Some(op) = matches.value_of("op") {
        //TODO: Check all arguments are set
        match op {
            "init" => {
                if let Err(e) = operations::init::setup(store_name) {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
            "create" => {
                println!("Create");
                if let Err(e) = operations::create::create_entry_point(store_name) {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
            "delete" => {
                if let Err(e) = operations::delete::delete_secret_store(store_name) {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
            "get" => {
                let entry_name= matches.value_of("entry_name").unwrap();
                if let Err(e) = operations::get::display_secret(store_name, entry_name){
                    println!("{}", e);
                    std::process::exit(1);
                }
            },
            "modify" => {
                let entry_name= matches.value_of("entry_name").unwrap();
                if let Err(e) = operations::get::display_secret(store_name, entry_name){
                    println!("{}", e);
                    std::process::exit(1);
                }
            },
            _ => println!("Must enter a valid operation"),
        }
    } else {
        println!("Must enter an operation");
        return;
    }
}
