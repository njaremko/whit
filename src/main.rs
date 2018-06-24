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

pub mod commands;
pub mod config;

use failure::Error;
use rocket;
use rocket::http::Status;
use rocket::response::Response;
use rocket_contrib::{Json, Value};

use crate::commands::addresses::*;
use crate::commands::neighbor::*;
use crate::commands::node_info::*;
use crate::commands::transactions::*;

pub const VERSION: &str = "0.0.1";

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
