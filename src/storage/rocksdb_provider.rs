use super::traits::PersistenceProvider;
use failure::Error;
use rocksdb::{IteratorMode, Options, DB};
use std::slice::Iter;

struct DbInstance {
    db: DB,
}

enum Columns {
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

impl Columns {
    pub fn column_str(&self) -> &'static str {
        use self::Columns::*;
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

    pub fn iter() -> Iter<'static, Columns> {
        use self::Columns::*;
        static COLUMNS: [Columns; 9] = [
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
        COLUMNS.into_iter()
    }

    pub fn column_str_list() -> [&'static str; 9] {
        [
            "transaction",
            "transaction-metadata",
            "milestone",
            "stateDiff",
            "address",
            "approvee",
            "bundle",
            "obsoleteTag",
            "tag",
        ]
    }
}

impl Default for DbInstance {
    fn default() -> DbInstance {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.increase_parallelism(4);
        DbInstance {
            db: DB::open_cf(&opts, "/iota_db", &Columns::column_str_list()).unwrap(),
        }
    }
}

impl DbInstance {
    pub fn current(&self) -> &DB {
        &self.db
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
}
