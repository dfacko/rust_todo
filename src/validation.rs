use actix_web::web;
use serde_json::*;

pub fn validate(item: &web::Json<Value>, keys: Vec<&str>) -> Option<Value> {
    let mut messages: Vec<Value> = vec![];
    for key in keys {
        if item[key] == Value::Null {
            // also check if its the required type
            messages.push(json!({ "message": format!("{} not found in body", key) }));
        }
    }
    if messages.len() == 0 {
        return None;
    } else {
        return Some(json!({ "errors:": messages }));
    }
}
