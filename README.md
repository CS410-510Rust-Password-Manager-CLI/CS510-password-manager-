# Password Manager CLI
Authors: Tareq Jallad, Haohan Jiang  

## Description
This project uses Rust to implement a command line password manager. This CLI application allows the user to
create, delete, list, and modify secret entries and secret stores, all through the command line.
For user security, all input passwords and usernames are encrypted by a randomly generated RSA Private key.

## Usage quick guide:
### Build
This project can be built with `cargo build`

and the binary will be located at `target/debug/password_manager`
### Terminology
`Secret Entry` - A secret entry is a named username and password pair

`Secret Store` - A list of secret entries

### Create a new store
        password_manager -s <NAME> init

### Add a new secret to the store
        password_manager -s <NAME> create

### List all secrets in a store
        password_manager -s <NAME> list

### Delete a secrets store
        password_manager -s <NAME> delete-store

### Delete a secret entry in a secrets store
        password_manager -s <NAME> delete-entry -e <ENTRY_NAME>

### Modify an existing entry in a secrets store:
        password_manager -s <NAME> modify -e <ENTRY_NAME>

### Get an existing entry in a secrets store:
        password_manager -s <NAME> get -e <ENTRY_NAME>

## Detailed guide:
### Directory structure
#### Top level `main.rs`
#### Operations
#### Generic
#### Models
#### Mocks

### Home directory for application
### How secrets are encrypted:
#### Hashed Values
#### RSA Encrypted Values
### Error handling

## Testing
We preformed integration testing by hand. We did not have a library crate and could not figure out a way
to import the project into an integration test module to live in the `/tests/` directory. We hand tested
each functionality to ensure functionality. 

Unit tests exist within files when appropriate for the function. Certain modules were mocked by selective 
imports with `#[cfg(test)]` and `#[cfg(not(test))]` macros.

## Post-project retrospective
We thought that most things in the project went pretty well. Nothing was too troublesome. 
The problem arose during unit testing of the library functions that the different 
components used. When researching how to unit test functions, most rust mocking frameworks limited you
to mocking only traits and the project was not built with traits in mind. 

Perhaps this was an un-rust like way to code the project. 

## License  
This project is available under the 
[MIT license](https://github.com/CS410-510Rust-Password-Manager-CLI/CS510-password-manager-/blob/main/LICENSE)


