//use super::rocksdb_provider::Column;
use failure::Error;

pub trait PersistenceProvider {
    fn save(&self, column: &str, index: &[u8], model: &[u8]) -> Result<(), Error>;
    fn delete(&self, column: &str, index: &[u8]) -> Result<(), Error>;
    fn update(&self, column: &str, index: &[u8], model: &[u8]) -> Result<(), Error>;
    fn exists(&self, column: &str, index: &[u8]) -> Result<bool, Error>;
    fn latest(&self, column: &str) -> Result<(Vec<u8>, Vec<u8>), Error>;
    // fn keys_with_missing_references();
    fn get(&self, column: &str, index: &[u8]) -> Result<Vec<u8>, Error>;
    // fn may_exist(&self, column: &str, index: &[u8]) -> Result<bool, Error>;
    // fn count() -> usize;
    // fn keys_starting_with();
    // fn seek();
    // fn next();
    // fn previous();
    fn first(&self, column: &str) -> Result<(Vec<u8>, Vec<u8>), Error>;
    fn save_batch(&self, models: &[(&str, Vec<u8>, Vec<u8>)]) -> Result<(), Error>;
    fn clear(&mut self, column: &str) -> Result<(), Error>;
    fn clear_metadata(&mut self) -> Result<(), Error>;
}
