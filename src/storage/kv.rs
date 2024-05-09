use crate::errors::StorageError as Error;
use crate::storage::traits::Storage;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, fs};

pub struct KvStore {
    db_path: PathBuf,
}

impl KvStore {
    pub fn new(db_path: PathBuf) -> Self {
        KvStore { db_path }
    }
}

impl Storage for KvStore {
    fn db_path(&self) -> PathBuf {
        self.db_path.clone()
    }

    fn open_db(&self) -> Result<sled::Db, Error> {
        sled::open(&self.db_path).map_err(|e| Error::from(e))
    }

    fn get<T>(&self, key: &str) -> Result<Option<T>, Error>
    where
        T: for<'a> Deserialize<'a>,
    {
        let db = self.open_db()?;
        let value = db.get(key.as_bytes())?;
        let result = value.map(|bytes| bincode::deserialize(&bytes).map_err(Error::from));
        result.transpose()
    }

    fn set<T>(&self, key: &str, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let db = self.open_db()?;
        let bytes = bincode::serialize(value)?;
        db.insert(key.as_bytes(), bytes.as_slice())
            .map_err(Error::from)
            .map(|_| ())
    }

    fn delete(&self, key: &str) -> Result<(), Error> {
        let db = self.open_db()?;
        db.remove(key.as_bytes())
            .map(|_| ())
            .map_err(Error::from)
    }

    fn clear(&self) -> Result<(), Error> {
        let db = self.open_db()?;
        db.clear().map_err(Error::from)
    }

    fn load_from_json<T>(&self, key: &str, path: PathBuf) -> Result<(), Error>
    where
        T: for<'a> Deserialize<'a> + Serialize,
    {
        let contents = fs::read_to_string(&path)?;
        let value = serde_json::from_str::<T>(&contents)?;
        self.set(key, &value)
    }
}