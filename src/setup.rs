use crate::storage::KvStore;
use std::path::PathBuf;

pub fn initialize_storage() -> KvStore {
    let db_path = PathBuf::from("./data/db");

    if !db_path.parent().unwrap().exists() {
        std::fs::create_dir_all(db_path.parent().unwrap())
            .expect("Failed to create storage directory");
    }

    KvStore::new(db_path)
}
