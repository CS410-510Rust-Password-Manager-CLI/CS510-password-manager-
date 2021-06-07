// Contains Error Classes for use through the app

#![allow(clippy::result_unit_err)]
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum PasswordStoreError<'a> {
    #[error("{0}: Password Store already exists")]
    PasswordStoreExists(&'a str),
    #[error("Could not get home directory")]
    HomeDirError,
    #[error("Could not create base path")]
    ErrorCreatingBasePath,
    #[error("Could not create store path")]
    ErrorCreatingStorePath,
    #[error("Must enter a store name")]
    ErrorNoStoreName,
    #[error("Entered passwords do not match! Passwords must match to create an entry!")]
    ErrorMisMatchPasswordCreation,
    #[error("Could not generate new RSA private key!")]
    ErrorPrivateKeyGeneration,
    #[error("Could not encrypt data")]
    ErrorDataEncryption,
    #[error("Store directory does not exist")]
    ErrorStoreDir,
    #[error("Store does not exist")]
    ErrorStoreExist,
    #[error("Entered store names do not match! Store names must match to delete an entry")]
    ErrorMisMatchStoreName,
}

pub type Result<'a, T> = std::result::Result<T, PasswordStoreError<'a>>;
