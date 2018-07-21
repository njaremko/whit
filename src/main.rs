#![allow(dead_code)]
#![feature(plugin)]
#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;

pub mod commands;
pub mod config;
pub mod model;
pub mod network;
pub mod signed_files;
pub mod snapshot;
pub mod storage;

use failure::Error;
use rocket;
use rocket::http::Status;
use rocket::response::Response;
use rocket_contrib::{Json, Value};
use reqwest::Client;

use crate::commands::*;

pub const VERSION: &str = "0.0.1";

lazy_static! {
    static ref IRI_URLS: [&'static str; 8] = {
        [
            "https://pow1.iota.community",
            "https://pow2.iota.community",
            "https://pow3.iota.community",
            "https://pow4.iota.community",
            "https://pow5.iota.community",
            "https://pow6.iota.community",
            "https://nodes.iota.fm",
            "https://trinity.iota.fm",
        ]
    };
    static ref CLIENT: reqwest::Client = {
            Client::new()
    };
}

#[derive(Serialize, Deserialize)]
pub struct IotaCommand {
    command: String,
    uris: Option<Vec<String>>,
    addresses: Option<Vec<String>>,
    hashes: Option<Vec<String>>,
    transactions: Option<Vec<String>>,
    tips: Option<Vec<String>>,
    trunk_transaction: Option<String>,
    branch_transaction: Option<String>,
    min_weight_magnitude: Option<u32>,
    trytes: Option<Vec<String>>,
}

#[post("/", data = "<request>")]
fn command_handler(request: Json<IotaCommand>) -> Result<Json<Value>, Error> {
    match request.command.as_ref() {
        "getNodeInfo" => get_node_info(),
        "getNeighbors" => get_neighbors(),
        "addNeighbors" => add_neighbors(&request),
        "removeNeighbors" => remove_neighbors(&request),
        "getTips" => get_tips(),
        "findTransactions" => find_transactions(&request),
        "getTrytes" => get_trytes(&request),
        "getInclusionStates" => get_inclusion_states(&request),
        "getBalances" => get_balances(&request),
        "getTransactionsToApprove" => get_transactions_to_approve(&request),
        "attachToTangle" => attach_to_tangle(&request),
        "broadcastTransactions" => broadcast_transactions(&request),
        "storeTransactions" => store_transactions(&request),
        "wereAddressesSpentFrom" => were_addresses_spent_from(&request),
        command => Err(format_err!("No command [{}] avaliable", command)),
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, command_handler])
        .launch();
}
