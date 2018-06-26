pub const SUPPLY: usize = 2779530283277761;
const SIZE: usize = 1604;

use serde_json;
use std::fmt;

#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    bytes: Vec<u8>,
    address: Option<String>,
    bundle: Option<String>,
    trunk_transaction: Vec<u8>,
    branch_transaction: Vec<u8>,
    obsolete_tag: Option<String>,
    value: usize,
    current_index: Option<usize>,
    last_index: Option<usize>,
    timestamp: Option<i64>,
    tag: Option<String>,
    attachment_timestamp: Option<i64>,
    attachment_timestamp_lower_bound: Option<i64>,
    attachment_timestamp_upper_bound: Option<i64>,
    validity: usize,
    arrival_time: usize,
    parsed: bool,
    solid: bool,
    height: usize,
    sender: String,
    snapshot: usize,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

impl Transaction {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    pub fn address(&self) -> &Option<String> {
        &self.address
    }
    pub fn address_mut(&mut self) -> &mut Option<String> {
        &mut self.address
    }

    pub fn bundle(&self) -> &Option<String> {
        &self.bundle
    }
    pub fn bundle_mut(&mut self) -> &mut Option<String> {
        &mut self.bundle
    }

    pub fn trunk_transaction(&self) -> &[u8] {
        &self.trunk_transaction
    }
    pub fn trunk_transaction_mut(&mut self) -> &mut [u8] {
        &mut self.trunk_transaction
    }

    pub fn branch_transaction(&self) -> &[u8] {
        &self.branch_transaction
    }
    pub fn branch_transaction_mut(&mut self) -> &mut [u8] {
        &mut self.branch_transaction
    }

    pub fn obsolete_tag(&self) -> &Option<String> {
        &self.obsolete_tag
    }
    pub fn obsolete_tag_mut(&mut self) -> &mut Option<String> {
        &mut self.obsolete_tag
    }

    pub fn value(&self) -> usize {
        self.value
    }
    pub fn value_mut(&mut self) -> &mut usize {
        &mut self.value
    }

    pub fn current_index(&self) -> &Option<usize> {
        &self.current_index
    }
    pub fn current_index_mut(&mut self) -> &mut Option<usize> {
        &mut self.current_index
    }

    pub fn last_index(&self) -> &Option<usize> {
        &self.last_index
    }
    pub fn last_index_mut(&mut self) -> &mut Option<usize> {
        &mut self.last_index
    }

    pub fn timestamp(&self) -> &Option<i64> {
        &self.timestamp
    }
    pub fn timestamp_mut(&mut self) -> &mut Option<i64> {
        &mut self.timestamp
    }

    pub fn tag(&self) -> &Option<String> {
        &self.tag
    }
    pub fn tag_mut(&mut self) -> &mut Option<String> {
        &mut self.tag
    }

    pub fn attachment_timestamp(&self) -> &Option<i64> {
        &self.attachment_timestamp
    }
    pub fn attachment_timestamp_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp
    }

    pub fn attachment_timestamp_lower_bound(&self) -> &Option<i64> {
        &self.attachment_timestamp_lower_bound
    }
    pub fn attachment_timestamp_lower_bound_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp_lower_bound
    }

    pub fn attachment_timestamp_upper_bound(&self) -> &Option<i64> {
        &self.attachment_timestamp_upper_bound
    }
    pub fn attachment_timestamp_upper_bound_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp_upper_bound
    }

    pub fn validity(&self) -> usize {
        self.validity
    }
    pub fn validity_mut(&mut self) -> &mut usize {
        &mut self.validity
    }

    pub fn arrival_time(&self) -> usize {
        self.arrival_time
    }
    pub fn arrival_time_mut(&mut self) -> &mut usize {
        &mut self.arrival_time
    }

    pub fn parsed(&self) -> bool {
        self.parsed
    }
    pub fn parsed_mut(&mut self) -> &mut bool {
        &mut self.parsed
    }

    pub fn solid(&self) -> bool {
        self.solid
    }
    pub fn solid_mut(&mut self) -> &mut bool {
        &mut self.solid
    }

    pub fn height(&self) -> usize {
        self.height
    }
    pub fn height_mut(&mut self) -> &mut usize {
        &mut self.height
    }

    pub fn sender(&self) -> &str {
        &self.sender
    }
    pub fn sender_mut(&mut self) -> &mut String {
        &mut self.sender
    }

    pub fn snapshot(&self) -> usize {
        self.snapshot
    }
    pub fn snapshot_mut(&mut self) -> &mut usize {
        &mut self.snapshot
    }
}
