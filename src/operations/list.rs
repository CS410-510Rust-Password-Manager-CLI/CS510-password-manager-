use crate::generic::common::{get_all_secrets, does_store_exist};
use crate::generic::errors::{PasswordStoreError, Result};

//List all secrets in a store
pub fn list_all_stores(store_name: &str) -> Result<()> {
    if !does_store_exist(store_name) {
        // Throw error if the requested store does not exist
        return Err(PasswordStoreError::ErrorStoreDoesNotExist);
    }

    match get_all_secrets(store_name) {
        Some(secrets) => {
            for entry in (*secrets).entries {
                println!("{}", entry.name);
            }
            Ok(())
        }
        None => Err(PasswordStoreError::ErrorStoreNoData),
    }
}
