// Contains Error Classes for use through the app

#[derive(Error, Debug, PartialEq)]
pub enum PasswordStoreError<'a> {
    #[error("{0}: Password Store already exists")]
    PasswordStoreExists(&'a str),
}

pub type Result<'a, T> = std::result::Result<T, PasswordStoreError<'a>>;