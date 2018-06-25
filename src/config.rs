use std::net::SocketAddr;

const MAINNET_COORDINATOR_ADDRESS: &str =
    "KPWCHICGJZXKE9GSUDXZYUAPLHAKAHYHDXNPHENTERYMMBQOPSQIDENXKLKCEYCPVTZQLEEJVYJZV9BWU";
const TESTNET_COORDINATOR_ADDRESS: &str =
    "EQQFCZBIHRHWPXKMTOLMYUYPCN9XLMJPYZVFJSAY9FQHCCLWTOLLUGKKMXYFDBOOYFBLBI9WUEILGECYM";
const MAINNET_SNAPSHOT_FILE: &str = "/snapshotMainnet.txt";
const TESTNET_SNAPSHOT_FILE: &str = "/snapshotTestnet.txt";
const MAINNET_SNAPSHOT_SIG_FILE: &str = "/snapshotMainnet.sig";

const PREVIOUS_EPOCHS_SPENT_ADDRESSES_TXT: &str = "/previousEpochsSpentAddresses.txt";
const PREVIOUS_EPOCH_SPENT_ADDRESSES_SIG: &str = "/previousEpochsSpentAddresses.sig";
const MAINNET_MILESTONE_START_INDEX: usize = 426550;
const TESTNET_MILESTONE_START_INDEX: usize = 434525;
const MAINNET_NUM_KEYS_IN_MILESTONE: usize = 20;
const TESTNET_NUM_KEYS_IN_MILESTONE: usize = 22;
const GLOBAL_SNAPSHOT_TIME: usize = 1525042800;
const TESTNET_GLOBAL_SNAPSHOT_TIME: usize = 1522306500;

const MAINNET_MWM: usize = 14;
const TESTNET_MWM: usize = 9;
const PACKET_SIZE: usize = 1650;
const TESTNET_PACKET_SIZE: usize = 1653;
const REQ_HASH_SIZE: usize = 46;
const TESTNET_REQ_HASH_SIZE: usize = 49;

pub struct Config {
    port: usize,
    api_host: String,
    udp_receiver_port: usize,
    tcp_receiver_port: usize,
    testnet: bool,
    debug: bool,
    remote_limit_api: Option<String>,
    remote_auth: Option<String>,
    neighbors: String,
    ixi_dir: String,
    db_path: String,
    db_log_path: String,
    db_cache_size: usize,
    config: String,
    p_remove_request: f64,
    p_drop_transaction: f64,
    p_select_milestone_child: f64,
    p_send_milestone: f64,
    p_reply_random_tip: f64,
    p_propagate_request: f64,
    main_db: String,
    export: bool,
    send_limit: f64,
    max_peers: usize,
    dns_refresher_enabled: bool,
    dns_resolver_enabled: bool,
    revalidate: bool,
    rescan_db: bool,
    min_weight_magnitude: usize,
    min_random_walks: usize,
    max_random_walks: usize,
    max_depth: usize,
    max_find_transactions: usize,
    max_requests_list: usize,
    max_get_trytes: usize,
    max_body_length: usize,
    zmq_enabled: bool,
    zmq_port: usize,
    zmq_ipc: String,
    zmq_threads: usize,
    q_size_node: usize,
    p_drop_cache_entry: f64,
    cache_size_bytes: usize,
    coordinator: &'static str,
    dont_validate_testnet_milestone_sig: bool,
    snapshot_file: &'static str,
    snapshot_signature_file: &'static str,
    milestone_start_index: usize,
    number_of_keys_in_a_milestone: usize,
    transaction_packet_size: usize,
    request_hash_size: usize,
    snapshot_time: usize,
    tip_selection_alpha: f64,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            port: 14600,
            api_host: "localhost".to_string(),
            udp_receiver_port: 14600,
            tcp_receiver_port: 15600,
            testnet: false,
            debug: false,
            remote_limit_api: None,
            remote_auth: None,
            neighbors: String::new(),
            ixi_dir: "ixi".to_string(),
            db_path: "mainnetdb".to_string(),
            db_log_path: "mainnet.log".to_string(),
            db_cache_size: 100000,
            config: "iota.ini".to_string(),
            p_remove_request: 0.01,
            p_drop_transaction: 0.0,
            p_select_milestone_child: 0.7,
            p_send_milestone: 0.02,
            p_reply_random_tip: 0.66,
            p_propagate_request: 0.01,
            main_db: "rocksdb".to_string(),
            export: false,
            send_limit: -1.0,
            max_peers: 0,
            dns_refresher_enabled: true,
            dns_resolver_enabled: true,
            revalidate: false,
            rescan_db: false,
            min_weight_magnitude: MAINNET_MWM,
            min_random_walks: 5,
            max_random_walks: 27,
            max_depth: 15,
            max_find_transactions: 100000,
            max_requests_list: 1000,
            max_get_trytes: 10000,
            max_body_length: 1000000,
            zmq_enabled: false,
            zmq_port: 5556,
            zmq_ipc: "ipc://iri".to_string(),
            zmq_threads: 2,
            q_size_node: 1000,
            p_drop_cache_entry: 0.02,
            cache_size_bytes: 15000,
            coordinator: MAINNET_COORDINATOR_ADDRESS,
            dont_validate_testnet_milestone_sig: false,
            snapshot_file: MAINNET_SNAPSHOT_FILE,
            snapshot_signature_file: MAINNET_SNAPSHOT_SIG_FILE,
            milestone_start_index: MAINNET_MILESTONE_START_INDEX,
            number_of_keys_in_a_milestone: MAINNET_NUM_KEYS_IN_MILESTONE,
            transaction_packet_size: PACKET_SIZE,
            request_hash_size: REQ_HASH_SIZE,
            snapshot_time: 0,
            tip_selection_alpha: 0.001,
        }
    }
}

impl Config {
    pub fn port(&self) -> usize {
        self.port
    }
    pub fn port_mut(&mut self) -> &mut usize {
        &mut self.port
    }
    pub fn api_host(&self) -> &str {
        &self.api_host
    }
    pub fn api_host_mut(&mut self) -> &mut String {
        &mut self.api_host
    }
}
