use crate::IotaCommand;
use failure::Error;
use rocket_contrib::{Json, Value};

use crate::VERSION;

pub fn get_node_info() -> Result<Json<Value>, Error> {
    Ok(Json(json!({
       "appName": "Whit",
       "appVersion": VERSION,
       "jreAvailableProcesses": 0,
       "jreFreeMemory": 0,
       "jreMaxMemory": 0,
       "jreTotalMemory": 0,
       "latestMilestone": "",
       "latestMilestoneIndex": 0,
       "latestSolidSubtangleMilestone": "",
       "latestSolidSubtangleMilestoneIndex": "",
       "neighbors": 0,
       "packetsQueueSize": 0,
       "time": 0,
       "tips": 0,
       "transactionsToRequest": "",
   })))
}

pub fn get_tips() -> Result<Json<Value>, Error> {
    Ok(Json(json!("")))
}

pub fn get_trytes(request: &IotaCommand) -> Result<Json<Value>, Error> {
    match &request.hashes {
        Some(hashes) => Ok(Json(json!(hashes))),
        None => Err(format_err!("No hashes provided")),
    }
}

pub fn get_inclusion_states(request: &IotaCommand) -> Result<Json<Value>, Error> {
    match &request.transactions {
        Some(transactions) => Ok(Json(json!(transactions))),
        None => Err(format_err!("No hashes provided")),
    }
}
