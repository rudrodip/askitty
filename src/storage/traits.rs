use crate::errors::StorageError as Error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub trait Storage {
    fn db_path(&self) -> PathBuf;

    fn open_db(&self) -> Result<sled::Db, Error>;
    fn get<T>(&self, key: &str) -> Result<Option<T>, Error>
    where
        T: for<'a> Deserialize<'a>;
    fn set<T>(&self, key: &str, value: &T) -> Result<(), Error>
    where
        T: Serialize;
    fn delete(&self, key: &str) -> Result<(), Error>;
    fn clear(&self) -> Result<(), Error>;
    fn load_from_json<T>(&self, key: &str, path: PathBuf) -> Result<(), Error>
    where
        T: for<'a> Deserialize<'a> + Serialize;
}
