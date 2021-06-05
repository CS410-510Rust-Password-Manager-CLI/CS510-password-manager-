use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/**
*   Hashes name input string
*   Returns str reference to hashed str name
*/
pub fn calculate_store_name_hash<T: Hash + ?Sized>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
