use failure::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;

const SNAPSHOT_PUBKEY: &str =
    "TTXJUGKTNPOOEXSTQVVACENJOQUROXYKDRCVK9LHUXILCLABLGJTIPNF9REWHOIMEUKWQLUOKD9CZUYAC";
const SNAPSHOT_PUBKEY_DEPTH: usize = 6;
const SNAPSHOT_INDEX: usize = 2;
const SPENT_ADDRESSES_INDEX: usize = 3;

struct Snapshot {
    state: RwLock<HashMap<Vec<u8>, usize>>,
    index: AtomicUsize,
}

impl Clone for Snapshot {
    fn clone(&self) -> Snapshot {
        let state = self.state.read().unwrap().clone();
        Snapshot {
            state: RwLock::new(state),
            index: AtomicUsize::new(self.index.load(Ordering::SeqCst)),
        }
    }
}

impl Snapshot {
    pub fn new(state: HashMap<Vec<u8>, usize>, index: usize) -> Snapshot {
        Snapshot {
            state: RwLock::new(state),
            index: AtomicUsize::new(index),
        }
    }

    pub fn init(
        &mut self,
        snapshot_path: &str,
        snapshot_sig_path: &str,
        testnet: bool,
    ) -> Result<Snapshot, Error> {
        use crate::signed_files;

        if !testnet
            && !signed_files::is_file_signature_valid(
                snapshot_path,
                snapshot_sig_path,
                SNAPSHOT_PUBKEY,
                SNAPSHOT_PUBKEY_DEPTH,
                SPENT_ADDRESSES_INDEX,
            )? {
            return Err(format_err!("Snapshot signature failed."));
        }
        let initial_state = init_initial_state(snapshot_path)?;
        let initial_snapshot = Snapshot::new(initial_state.clone(), 0);
        check_state_has_correct_supply(&initial_state)?;
        Ok(initial_snapshot)
    }

    pub fn index(&self) -> usize {
        self.index.load(Ordering::SeqCst)
    }

    pub fn balance(&self, hash: &[u8]) -> Option<usize> {
        if let Some(t) = self.state.read().unwrap().get(hash) {
            return Some(t.clone());
        }
        None
    }

    pub fn patched_diff(&self, diff: HashMap<Vec<u8>, usize>) -> HashMap<Vec<u8>, usize> {
        diff.iter()
            .map(|(key, val)| match self.state.read().unwrap().get(key) {
                Some(t) => (key.clone(), t + val),
                None => (key.clone(), val.clone()),
            })
            .collect()
    }

    pub fn apply(&self, patch: HashMap<Vec<u8>, usize>, new_index: usize) -> Result<(), Error> {
        ensure!(patch.iter().map(|(_, val)| val).sum::<usize>() == 0, "Diff is not consistent.");
        for (key, val) in patch {
            match self.state.read().unwrap().get(&key) {
                Some(t) => self.state.write().unwrap().insert(key, t + val),
                None => self.state.write().unwrap().insert(key, val),
            };
        }
        self.index.store(new_index, Ordering::SeqCst);
        Ok(())
    }
}

fn init_initial_state(snapshot_file: &str) -> Result<HashMap<Vec<u8>, usize>, Error> {
    let mut state: HashMap<Vec<u8>, usize> = HashMap::new();
    let f = File::open(snapshot_file)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let parts: Vec<String> = line?.split(';').take(2).map(|x| x.to_string()).collect();
        if parts.len() >= 2 {
            state.insert(parts[0].as_bytes().to_vec(), parts[1].parse()?);
        }
    }
    Ok(state)
}

fn check_state_has_correct_supply(initial_state: &HashMap<Vec<u8>, usize>) -> Result<(), Error> {
    use crate::model::transaction::SUPPLY;
    let state_value: usize = initial_state.values().sum();
    ensure!(
        state_value == SUPPLY,
        "Transaction resolves to incorrect ledger balance: {}",
        SUPPLY - state_value
    );
    Ok(())
}
