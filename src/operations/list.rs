use crate::generic::common::{does_store_exist, get_entry_names, get_path};
use crate::generic::errors::{PasswordStoreError, Result};

//List all secrets in a store
pub fn list_all_entries(store_name: &str) -> Result<()> {
    if !does_store_exist(store_name) {
        // Throw error if the requested store does not exist
        return Err(PasswordStoreError::ErrorStoreDoesNotExist);
    }

    // Get a list of all entry names
    // If none, return no data error
    let path = *get_path(store_name);
    match get_entry_names(&(path)) {
        Some(entry_names) => {
            for name in entry_names.iter() {
                println!("{}", name);
            }
            Ok(())
        }
        None => Err(PasswordStoreError::ErrorStoreNoData),
    }
}
