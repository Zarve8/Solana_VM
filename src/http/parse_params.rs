use std::fmt::Debug;
use serde::de::DeserializeOwned;
use crate::http::interfaces::params::optional::OptionalParams;
use crate::http::interfaces::rpc_request::RpcRequest;


pub fn parse_params<'a, T: DeserializeOwned + RpcRequest + Debug>(mut params: Vec<serde_json::value::Value>) -> (T, Option<OptionalParams>) {
    let request_body = &params[0..T::length()];
    println!("Request Body: {:?}", request_body);
    let parsed_body: T = serde_json::from_str::<T>(
        &serde_json::to_string(request_body).unwrap()
    ).expect("Failed to parse params");
    println!("Request Body {:?}", parsed_body);
    let optional_params: Option<OptionalParams> = {
        if params.len() > T::length() {
            Some(serde_json::from_value::<OptionalParams>(params.pop().unwrap()).unwrap())
        }
        else {
            None
        }
    };
    println!("Optional: {:?}", optional_params);
    (parsed_body, None)
}