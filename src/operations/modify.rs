use crate::generic::common::{calculate_store_name_hash, does_store_exist, write_to_file, get_path, get_all_secrets_from_store};
use crate::generic::errors::{PasswordStoreError, Result};

use crate::operations::delete::delete_entry;

use crate::operations::create::{add_to_store, get_password, get_username};

// Modify a data entry
pub fn modify_entry<'a>(store_name: &str, entry_name: &str) -> Result<'a, ()> {
    // Password protection around rotation
    if !does_store_exist(store_name) {
        // Throw error if the requested store does not exist
        return Err(PasswordStoreError::ErrorStoreDoesNotExist);
    }

    let path = get_path(store_name);
    let failback_copy = get_all_secrets_from_store(&path).unwrap();
    match delete_entry(store_name, entry_name) {
        Ok(()) => {
            let username: String = *get_username();
            let password: String = *(get_password().unwrap());
            let keyname = calculate_store_name_hash(entry_name).to_string();

            match add_to_store(&keyname, &username, &password, store_name, entry_name) {
                Ok(()) => Ok(()),
                Err(e) => {
                    // Fail back if error occurs with modifying password after
                    // the entry has already been removed
                    match write_to_file(
                        &(*failback_copy),
                        &calculate_store_name_hash(entry_name).to_string(),
                    ) {
                        Ok(()) => Err(e),
                        Err(e) => Err(e),
                    }
                }
            }
        }
        Err(e) => Err(e),
    }
}
