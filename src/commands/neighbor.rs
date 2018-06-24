use crate::IotaCommand;
use failure::Error;
use rocket_contrib::{Json, Value};

use iota_lib_rs::model::neighbor::Neighbor;

pub fn get_neighbors() -> Result<Json<Value>, Error> {
    Ok(Json(json!({
       "address": "",
       "numberOfAllTransactions": 0,
       "numberOfInvalidTransactions": 0,
       "numberOfNewTransactions": 0,
   })))
}

pub fn add_neighbors(request: &IotaCommand) -> Result<Json<Value>, Error> {
    match &request.uris {
        Some(uris) => {
            for uri in uris {
                let mut neighbor = Neighbor::default();
                *neighbor.address_mut() = uri.parse()?;
            }
            Ok(Json(json!("")))
        }
        None => Err(format_err!("No URIs provided")),
    }
}

pub fn remove_neighbors(request: &IotaCommand) -> Result<Json<Value>, Error> {
    match &request.uris {
        Some(uris) => Ok(Json(json!(uris))),
        None => Err(format_err!("No URIs provided")),
    }
}
