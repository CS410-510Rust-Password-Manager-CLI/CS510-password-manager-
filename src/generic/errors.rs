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
    #[error("Did not create a store")]
    ErrorCouldNotCreateStore,
    #[error("Entered passwords do not match! Passwords must match to create an entry!")]
    ErrorMisMatchPasswordCreation,
    #[error("Could not generate new RSA private key!")]
    ErrorPrivateKeyGeneration,
    #[error("Could not encrypt data")]
    ErrorDataEncryption,
    #[error("Entry does not exist")]
    ErrorStoreEntryDoesNotExist,
    #[error("Store does not exist")]
    ErrorStoreDoesNotExist,
    #[error("Entered store names do not match! Store names must match to delete an entry")]
    ErrorMisMatchStoreName,
    #[error("Entered entry name does not exist in this store!")]
    ErrorEntryDoesNotExist,
    #[error("No matching entry name found")]
    ErrorNoEntryNameMatch,
    #[error("Could not encrypt data")]
    ErrorEncryptionError,
    #[error("Store does not have any data!")]
    ErrorStoreNoData,
    #[error("Could not delete RSA Private Key!")]
    ErrorRSAKeyDelete,
    #[error("Entry already exists!")]
    ErrorEntryAlreadyExists,
}

pub type Result<'a, T> = std::result::Result<T, PasswordStoreError<'a>>;
