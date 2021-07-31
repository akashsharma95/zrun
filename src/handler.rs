use axum::{prelude::*, response::Json};
use bytes;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::executor;

pub async fn fn_handler(
    // Extract captured parameters from the URL
    params: Option<extract::UrlParamsMap>,
    // Parse query string into a `HashMap`
    query_params: Option<extract::Query<HashMap<String, String>>>,
    // Buffer the request body into a `Bytes`
    bytes: Option<bytes::Bytes>,
) -> Json<Value> {
    if let Some(params) = params {
        println!("{:?}", params)
    }

    if let Some(query_params) = query_params {
        if let Ok(v) = executor::execute(
            query_params
                .0
                .get("x")
                .unwrap_or(&String::from("10"))
                .parse::<i32>()
                .unwrap(),
            query_params
                .0
                .get("y")
                .unwrap_or(&String::from("10"))
                .parse::<i32>()
                .unwrap(),
        ) {
            return Json(json!({ "data": v }))
        } else {
            return Json(json!({ "data":  "error" }))
        };
    }

    if let Some(bytes) = bytes {
        println!("{:?}", bytes)
    }

    return Json(json!({ "data":  "error" }))
}

