use std::collections::HashMap;
use failure::Error;

pub trait WalkValidator {
    fn is_valid(transaction_hash: &[i8]) -> bool;
}

pub trait Walker {
    fn walk(entry_point: &[i8], ratings: HashMap<Vec<u8>, i32>, walkValidator: &impl WalkValidator) -> Result<Vec<i8>, Error>;
}

pub trait TipSelector {
    fn get_transactions_to_approve(depth: usize, reference: Option<Vec<i8>>) -> Result<Vec<Vec<i8>>, Error>;
    fn get_max_depth() -> usize;
}

pub trait TailFinder {
    fn find_tail(hash: Vec<i8>) -> Result<Option<Vec<i8>>, Error>;
}

pub trait RatingCalculator {
    fn calculate(entry_point: Vec<i8>) -> Result<HashMap<Vec<i8>, i32>, Error>;
}

pub trait EntryPointSelector {
    fn get_entry_point(depth: usize) -> Result<Vec<i8>, Error>;
}