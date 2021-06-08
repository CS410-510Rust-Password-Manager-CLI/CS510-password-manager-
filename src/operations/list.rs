use crate::generic::common::{get_entry_names, does_store_exist};
use crate::generic::errors::{PasswordStoreError, Result};

//List all secrets in a store
pub fn list_all_entries(store_name: &str) -> Result<()> {
    if !does_store_exist(store_name) {
        // Throw error if the requested store does not exist
        return Err(PasswordStoreError::ErrorStoreDoesNotExist);
    }

    match get_entry_names(store_name) {
        Some(entry_names) => {
            for name in entry_names.iter(){
                println!("{}", name);
            }
            Ok(())
        }
        None => Err(PasswordStoreError::ErrorStoreNoData),
    }
}
