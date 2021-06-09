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
### Home directory for application
### How secrets are encrypted
#### Hashed Values
#### RSA Encrypted Values
### Error handling

## Testing

## Post-project retrospective


## License  
This project is available under the 
[MIT license](https://github.com/CS410-510Rust-Password-Manager-CLI/CS510-password-manager-/blob/main/LICENSE)


