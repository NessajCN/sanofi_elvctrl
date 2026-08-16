use axum::{Extension, Json, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::ControlChannel;

const STX: u8 = 0x02;
const ADDR1: u8 = 0x30; // "0"
const ADDR2: u8 = 0x31; // "1"
const MAIN_CMD: u8 = 0x4d; // "M"
const ETX: u8 = 0x03;

fn gen_checksum(cmd: &mut Vec<u8>) {
    let sum = cmd.iter().clone().fold(0u8,|acc, &x| acc.wrapping_add(x));
    let sum_str = format!("{:02X}", sum);
    let checksum = sum_str.as_bytes();
    cmd.extend(checksum);
    cmd.push(ETX);
}

pub async fn elevator_control(
    Extension(control_channel): Extension<ControlChannel>,
    // this argument tells axum to parse the request body
    // as JSON into a `ElevCmd` type
    Json(command): Json<ElevCmd>,
) -> (StatusCode, Json<Response>) {
    // insert your application logic here
    match command.control_type {
        ControlType::Query => {
            let mut cmd = vec![STX, ADDR1, ADDR2, MAIN_CMD, ControlType::Query as u8];
            let data_vec = [0x30; 10];
            cmd.extend(data_vec);
            gen_checksum(&mut cmd);
            control_channel.send(cmd);
        }
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
