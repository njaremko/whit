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

use failure::Error;
use rocket;
use rocket::http::Status;
use rocket::response::Response;
use rocket_contrib::{Json, Value};

use crate::commands::node_info::get_node_info;
use crate::commands::neighbor::*;
use crate::commands::transactions::*;
use crate::commands::addresses::*;

static PHRASE: &'static [u8] = b"Hello World!";
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
fn index(request: Json<IotaCommand>) -> Result<Json<Value>, Error> {
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

fn get_tips() -> Result<Json<Value>, Error> {
    Ok(Json(json!("")))
}

fn get_trytes(request: &Json<IotaCommand>) -> Result<Json<Value>, Error> {
    match &request.hashes {
        Some(hashes) => Ok(Json(json!(hashes))),
        None => Err(format_err!("No hashes provided")),
    }
}

fn get_inclusion_states(request: &Json<IotaCommand>) -> Result<Json<Value>, Error> {
    match &request.transactions {
        Some(transactions) => Ok(Json(json!(transactions))),
        None => Err(format_err!("No hashes provided")),
    }
}

fn interrupt_attaching_to_tangle() -> Result<Json<Value>, Error> {
        Ok(Json(json!("")))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
