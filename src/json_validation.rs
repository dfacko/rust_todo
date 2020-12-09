use actix_web::web;
use serde_json::*;
use std::result::Result;
pub fn validate(item: &web::Json<Value>, keys: Vec<&str>) -> Option<Value> {
    let mut messages: Vec<Value> = vec![];
    for key in keys {
        let rule = String::from(key);
        let mut rules: Vec<&str> = rule.split('|').collect();
        let value_key = rules.remove(0);
        for rule in &rules {
            if item[value_key] == Value::Null {
                messages.push(json!({
                    "message": format!("{} not found in body", value_key)
                }));
            } else {
                match checktype(&item[value_key], rule) {
                    Err(_) => {
                        messages.push(json!({
                            "message": format!("{} is not of required type ({})", value_key, rule)
                        }));
                    }
                    _ => {}
                }
            }
        }
    }
    if messages.len() == 0 {
        return None;
    } else {
        return Some(json!({ "errors:": messages }));
    }
}

fn checktype(item: &Value, rule: &str) -> Result<(), ()> {
    let value_type = match item {
        Value::Number(_) => "int",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
        Value::Bool(_) => "bool",
        _ => "not a value",
    };

    if rule != value_type {
        return Err(());
    };
    Ok(())
}
