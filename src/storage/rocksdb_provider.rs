use super::traits::PersistenceProvider;
use failure::Error;
use rocksdb::{IteratorMode, Options, WriteBatch, DB};
use std::slice::Iter;

/// This module provides an implementation of the PersistanceProvider
/// trait for the RocksDB backend.

/// DbInstance holds a reference to the rocksdb instance,
/// If DbInstance goes out of scope, the rocksdb instance is
/// automatically closed
pub struct DbInstance {
    db: DB,
}

/// This is used to prevent spelling mistakes when referencing
/// column families
pub enum Column {
    Transaction,
    TransactionMetadata,
    Milestone,
    StateDiff,
    Address,
    Approvee,
    Bundle,
    ObsoleteTag,
    Tag,
}

impl Column {
    pub fn column_str(&self) -> &'static str {
        use self::Column::*;
        match self {
            Transaction => "transaction",
            TransactionMetadata => "transaction-metadata",
            Milestone => "milestone",
            StateDiff => "stateDiff",
            Address => "address",
            Approvee => "approvee",
            Bundle => "bundle",
            ObsoleteTag => "obsoleteTag",
            Tag => "tag",
        }
    }

    pub fn iter() -> Iter<'static, Column> {
        use self::Column::*;
        static COLUMN: [Column; 9] = [
            Transaction,
            TransactionMetadata,
            Milestone,
            StateDiff,
            Address,
            Approvee,
            Bundle,
            ObsoleteTag,
            Tag,
        ];
        COLUMN.into_iter()
    }

    pub fn column_str_list() -> Vec<&'static str> {
        Column::iter().map(|c| c.column_str()).collect()
    }
}

/// Used to open the db with reasonable defaults
impl Default for DbInstance {
    fn default() -> DbInstance {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.increase_parallelism(4);
        DbInstance {
            db: DB::open_cf(&opts, "/iota_db", &Column::column_str_list()).unwrap(),
        }
    }
}

impl PersistenceProvider for DbInstance {
    fn save(&self, column: &str, index: &[u8], model: &[u8]) -> Result<(), Error> {
        if let Some(handle) = self.db.cf_handle(column) {
            self.db.put_cf(handle, index, model)?;
            return Ok(());
        }
        Err(format_err!("No column with that name"))
    }
    fn delete(&self, column: &str, index: &[u8]) -> Result<(), Error> {
        if let Some(handle) = self.db.cf_handle(column) {
            self.db.delete_cf(handle, index)?;
            return Ok(());
        }
        Err(format_err!("No column with that name"))
    }
    fn update(&self, column: &str, index: &[u8], model: &[u8]) -> Result<(), Error> {
        if let Some(handle) = self.db.cf_handle(column) {
            self.db.put_cf(handle, index, model)?;
            return Ok(());
        }
        Err(format_err!("No column with that name"))
    }
    fn exists(&self, column: &str, index: &[u8]) -> Result<bool, Error> {
        if let Some(handle) = self.db.cf_handle(column) {
            return match self.db.get_cf(handle, index)? {
                Some(_) => Ok(true),
                None => Ok(false),
            };
        }
        Err(format_err!("No column with that name"))
    }

    fn latest(&self, column: &str) -> Result<(Vec<u8>, Vec<u8>), Error> {
        if let Some(handle) = self.db.cf_handle(column) {
            let iter = self.db.iterator_cf(handle, IteratorMode::End)?;
            for (key, value) in iter.take(1) {
                return Ok((key.to_vec(), value.to_vec()));
            }
        }
        Err(format_err!("No column with that name"))
    }

    fn first(&self, column: &str) -> Result<(Vec<u8>, Vec<u8>), Error> {
        if let Some(handle) = self.db.cf_handle(column) {
            let iter = self.db.iterator_cf(handle, IteratorMode::Start)?;
            for (key, value) in iter.take(1) {
                return Ok((key.to_vec(), value.to_vec()));
            }
        }
        Err(format_err!("No column with that name"))
    }

    fn get(&self, column: &str, index: &[u8]) -> Result<Vec<u8>, Error> {
        if let Some(handle) = self.db.cf_handle(column) {
            if let Some(db_vec) = self.db.get_cf(handle, index)? {
                return Ok(db_vec.to_vec());
            }
            return Err(format_err!("No matching value in column: {}", column));
        }
        Err(format_err!("No column with that name"))
    }

    fn clear(&mut self, column: &str) -> Result<(), Error> {
        self.flush_handle(column)?;
        Ok(())
    }

    fn clear_metadata(&mut self) -> Result<(), Error> {
        self.flush_handle(Column::TransactionMetadata.column_str())?;
        Ok(())
    }

    fn save_batch(&self, models: &[(&str, Vec<u8>, Vec<u8>)]) -> Result<(), Error> {
        let mut batch = WriteBatch::default();
        for (column, key, value) in models.into_iter() {
            match self.db.cf_handle(column) {
                Some(handle) => batch.put_cf(handle, key, value)?,
                None => return Err(format_err!("No column with that name")),
            }
        }
        self.db.write(batch)?;
        Ok(())
    }
}

impl DbInstance {
    pub fn current(&self) -> &DB {
        &self.db
    }

    fn flush_handle(&self, column: &str) -> Result<(), Error> {
        if let Some(handle) = self.db.cf_handle(column) {
            let mut batch = WriteBatch::default();
            let iter = self.db.iterator_cf(handle, IteratorMode::Start)?;
            for (key, _) in iter {
                batch.delete_cf(handle, &key)?
            }
            self.db.write(batch)?;
            return Ok(());
        }
        Err(format_err!("No column with that name"))
    }
}
