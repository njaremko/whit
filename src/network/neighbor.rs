use std::net::{IpAddr, SocketAddr};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
pub struct Neighbor {
    address: SocketAddr,
    number_of_all_transactions: usize,
    number_of_new_transactions: usize,
    number_of_invalid_transactions: usize,
    random_transaction_requests: usize,
    number_of_sent_transactions: usize,
    flagged: bool,
    num_peers: AtomicUsize,
    host_address: IpAddr,
}

impl Neighbor {
    pub fn new(address: SocketAddr, is_configured: bool) -> Neighbor {
        Neighbor {
            address: address,
            number_of_all_transactions: 0,
            number_of_new_transactions: 0,
            number_of_invalid_transactions: 0,
            random_transaction_requests: 0,
            number_of_sent_transactions: 0,
            flagged: is_configured,
            num_peers: AtomicUsize::new(0),
            host_address: address.ip(),
        }
    }

    pub fn flagged(&self) -> bool {
        self.flagged
    }
    pub fn flagged_mut(&mut self) -> &mut bool {
        &mut self.flagged
    }

    pub fn increment_all_transactions(&mut self) {
        self.number_of_all_transactions += 1;
    }
    pub fn number_of_all_transactions(&self) -> usize {
        self.number_of_all_transactions
    }

    pub fn increment_new_transactions(&mut self) {
        self.number_of_new_transactions += 1;
    }
    pub fn number_of_new_transactions(&self) -> usize {
        self.number_of_new_transactions
    }

    pub fn increment_random_transaction_requests(&mut self) {
        self.random_transaction_requests += 1;
    }
    pub fn random_transaction_requests(&self) -> usize {
        self.random_transaction_requests
    }

    pub fn increment_sent_transactions(&mut self) {
        self.number_of_sent_transactions += 1;
    }
    pub fn number_of_sent_transactions(&self) -> usize {
        self.number_of_sent_transactions
    }

    pub fn increment_invalid_transactions(&mut self) {
        self.number_of_invalid_transactions += 1;
    }
    pub fn number_of_invalid_transactions(&self) -> usize {
        self.number_of_invalid_transactions
    }

    pub fn increment_num_peers(&self) {
        let i = self.num_peers.load(Ordering::SeqCst);
        self.num_peers.store(i + 1, Ordering::SeqCst);
    }
    pub fn decrement_num_peers(&self) {
        let i = self.num_peers.load(Ordering::SeqCst);
        if i > 0 {
            self.num_peers.store(i - 1, Ordering::SeqCst);
        }
    }
    pub fn num_peers(&self) -> usize {
        self.num_peers.load(Ordering::SeqCst)
    }
}
