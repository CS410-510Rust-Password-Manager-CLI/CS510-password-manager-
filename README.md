# Password Manager CLI
Authors: Tareq Jallad, Haohan Jiang  

## Guide
- [Description](#description)
- [Usage](#usage-quick-guide)
  - [Build](#build)
  - [Terminology](#terminology)
  - [Create a new store](#create-a-new-store)
  - [Add a secret to store](#add-a-new-secret-to-the-store)
  - [List all secrets in a store](#list-all-secrets-in-a-store)
  - [Delete a secrets store](#delete-a-secrets-store)
  - [Delete a secret entry in a secrets store](#delete-a-secret-entry-in-a-secrets-store)
  - [Modify an existing entry in a secrets store](#modify-an-existing-entry-in-a-secrets-store)
  - [Get an existing entry in a secrets store](#get-an-existing-entry-in-a-secrets-store)
- [Detailed guide](#detailed-guide)
  - [Directory structure](#directory-structure)
  - [Home directory for application](#home-directory-for-application)
  - [How secrets are encrypted](#how-secrets-are-encrypted)
  - [Hashed Values](#hashed-values)
  - [RSA Encrypted Values](#rsa-encrypted-values)
  - [Error handling](#error-handling)
- [Testing](#testing)
- [Post-project retrospective](#post-project-retrospective)

## Description
This project uses Rust to implement a command line password manager. This CLI application allows the user to
create, delete, list, and modify secret entries and secret stores, all through the command line.
For user security, all input passwords and usernames are encrypted by a randomly generated RSA Private key.

## Usage quick guide
### Build
This project can be built with `cargo build`

and the binary will be located at `target/debug/password_manager`  

Binary can also be downloaded from the [release page](https://github.com/CS410-510Rust-Password-Manager-CLI/CS510-password-manager-/releases)
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

### Modify an existing entry in a secrets store
        password_manager -s <NAME> modify -e <ENTRY_NAME>

### Get an existing entry in a secrets store
        password_manager -s <NAME> get -e <ENTRY_NAME>

## Detailed guide:
### Directory structure
#### Top level `main.rs`
The entry point for the application. Houses command line argument logic and calls different
modules based on user input.

#### Operations `/operations/`
- `create.rs`
- `delete.rs`
- `get.rs`
- `init.rs`
- `list.rs`
- `modify.rs`

Operations directory contains the logic required for each specific operation. 

#### Generic `/generic/`
- `common.rs`
- `encryption.rs`
- `errors.rs`

Generics directory contains functionality that is used across the app. 

#### Models `/models/`
-`data_model.rs`

Models directory contains the data model that is serialized and deserialized through JSON.

#### Mocks `/mocks/`
-`test_mocks.rs`

Mocks directory contains a function that is used to override an external function during unit testing.
`test_mocks.rs` contains a function that is only called during unit tests which raises a compiler warning during
`build`. So, we decided to suppress the warning with the macro `#![allow(dead_code)]` 

### Home directory for application
The base directory for this application is: `HOME/.passmanager`.

All secrets stores are located at: `HOME/.passmanager/.stores/<STORE_NAME>.json`

All secret keys are located at: `HOME/.passmanager/.keys/<KEY_NAME>.pem`

### How secrets are encrypted
Secrets and secret stores are encrypted in two different ways. First, we encrypt the store name and the entry name.
Second, we also encrypt the username and password for each entry using a RSA private key generated specifically for that
secret entry. 

#### Hashed Values
Secret store names and RSA private keys are stored under hashed value. 

#### RSA Encrypted Values
Usernames and passwords are encrypted by a RSA public key generated from it's RSA private key. The key is stored under
the hashed entry name so that the private key can be retrieved to decrypt the username and password. 

### Error handling
Application errors can handled under `/generics/errors`. External library errors are caught and re-raised as an internal
error up to the top level caller. 

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


