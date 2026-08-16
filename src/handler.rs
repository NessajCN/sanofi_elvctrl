use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

pub async fn elevator_control(
    // this argument tells axum to parse the request body
    // as JSON into a `ElevCmd` type
    Json(command): Json<ElevCmd>,
) -> (StatusCode, Json<Response>) {
    // insert your application logic here
    match command.control_type {
        ControlType::Query => {}
        ControlType::Register => {}
        ControlType::Open => {}
        ControlType::Close => {}
        ControlType::Switch => {}
    }
    let resp = Response {
        success: false,
        message: "Command success".to_string(),
    };

    // this will be converted into a JSON response
    // with a status code of `200 Ok`
    (StatusCode::OK, Json(resp))
}

// the input to our `create_user` handler
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElevCmd {
    control_type: ControlType,
    front_floor: Option<u8>,
    back_floor: Option<u8>,
    frontdoor_state: Option<DoorState>,
    backdoor_state: Option<DoorState>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ControlType {
    // '3' as query
    Query = 0x33,
    // '5' as register
    Register = 0x35,
    // '6' as open
    Open = 0x36,
    // '7' as close
    Close = 0x37,
    // '8' as switch mode
    Switch = 0x38,
}

#[derive(Deserialize, Debug)]
pub enum DoorState {
    Open = 0x31,
    Close = 0x30,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct Response {
    success: bool,
    message: String,
}
