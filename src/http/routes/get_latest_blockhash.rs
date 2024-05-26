use serde_json::{json, Value};
use crate::global::state::GLOBAL_STATE;
use bs58;

pub async fn get_latest_blockhash() -> Value {
    let state = GLOBAL_STATE.lock().expect("Failed to read Global State");
    json!({
        "blockhash": state.blockhash,
        "lastValidBlockHeight": state.vars.slot
    })
}

/*
value: {
      blockhash: 'AKfrPRqEJNgza7TbzX4uSHjNuzhSshzURr7UMveJVkAw',
      lastValidBlockHeight: 286675255
    }
 */