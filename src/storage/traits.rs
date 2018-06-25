use failure::Error;

pub trait PersistenceProvider {
    //fn init();
    //fn is_available() -> bool;
    //fn shutdown();
    fn save(&self, column: &str, index: &[u8], model: &[u8]) -> Result<(), Error>;
    fn delete(&self, column: &str, index: &[u8]) -> Result<(), Error>;
    fn update(&self, column: &str, index: &[u8], model: &[u8]) -> Result<(), Error>;
    fn exists(&self, column: &str, index: &[u8]) -> Result<bool, Error>;
    fn latest(&self, column: &str) -> Result<(Vec<u8>, Vec<u8>), Error>;
    // fn keys_with_missing_references();
    fn get(&self, column: &str, index: &[u8]) -> Result<Vec<u8>, Error>;
    // fn may_exist(index: &[u8]) -> bool;
    // fn count() -> usize;
    // fn keys_starting_with();
    // fn seek();
    // fn next();
    // fn previous();
    fn first(&self, column: &str) -> Result<(Vec<u8>, Vec<u8>), Error>;
    // fn save_batch(models: &[(Vec<u8>, Vec<u8>)]) -> bool;
    // fn clear();
    // fn clear_metadata();
}
