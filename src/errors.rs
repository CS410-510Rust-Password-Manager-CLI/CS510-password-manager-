#![allow(clippy::result_unit_err)]

// Contains Error Classes for use through the app
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
    ErrorNoStoreName
}

pub type Result<'a, T> = std::result::Result<T, PasswordStoreError<'a>>;