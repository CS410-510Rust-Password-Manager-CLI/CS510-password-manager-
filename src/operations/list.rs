use crate::generic::common::get_all_secrets;
use crate::generic::errors::{
    PasswordStoreError,
    Result
};

//List all secrets in a store
pub fn list_all_stores(store_name: &str) -> Result<()>{
    match get_all_secrets(store_name){
        Some(secrets) => {
            for entry in (*secrets).entries{
                println!("{}", entry.name);
            }
            Ok(())
        }
        None => Err(PasswordStoreError::ErrorStoreNoData)
    }
}