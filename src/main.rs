use clap::{App, Arg};

// Internal Libraries
pub mod operations {
    pub mod create;
    pub mod delete;
    pub mod get;
    pub mod init;
    pub mod list;
    pub mod modify;
}

pub mod models {
    pub mod data_model;
}

pub mod generic {
    pub mod common;
    pub mod encryption;
    pub mod errors;
}

// Mock Library for unit testing
pub mod mocks {
    pub mod test_mocks;
}

pub fn main() {
    // Get all command line args
    let matches = App::new("Password Manager")
        .version("1.0")
        .author("Haohan Jiang <jiang4.pdx.edu>, Taraq Jallad <tajallad@pdx.edu>")
        .about(
            "Usage: \n\
        To initialize a new store: password_manager -s <NAME> init\n\
        To add a secret to the store: password_manager -s <NAME> create\n\
        To list all secrets in a store: password_manager -s <NAME> list\n\
        To delete a secrets store: password_manager -s <NAME> delete-store\n\
        To delete an entry in a secrets store: password_manager -s <NAME> delete-entry -e <ENTRY_NAME>\n\
        To modify an entry in a secrets store: password_manager -s <NAME> modify -e <ENTRY_NAME>\n\
        To get an entry from the secrets store: password_manager -s <NAME> get -e <ENTRY_NAME>\n",
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
        match op {
            "init" => {
                if let Err(e) = operations::init::setup(store_name) {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
            "create" => {
                if let Err(e) = operations::create::create_entry_point(store_name) {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
            "delete-store" => {
                if let Err(e) = operations::delete::delete_secret_store(store_name) {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
            "delete-entry" => {
                let entry_name = matches.value_of("entry_name").unwrap();
                if let Err(e) = operations::delete::delete_entry(store_name, entry_name) {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
            "get" => {
                let entry_name = matches.value_of("entry_name").unwrap();
                if let Err(e) = operations::get::display_secret(store_name, entry_name) {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
            "modify" => {
                let entry_name = matches.value_of("entry_name").unwrap();
                if let Err(e) = operations::modify::modify_entry(store_name, entry_name) {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
            "list" => {
                if let Err(e) = operations::list::list_all_entries(store_name) {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
            _ => println!("Must enter a valid operation"),
        }
    } else {
        println!("Must enter an operation");
    }
}
