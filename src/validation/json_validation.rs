use crate::error::*;
use crate::models::models::Mytrait;
use crate::models::models::UserNew;
use actix_web::web;
use serde::Deserialize;
use serde_json::{json, to_string, Value};
use std::{fmt::Debug, result::Result};
pub fn validate<T>(item: web::Json<Value>, keys: Vec<&str>) -> Result<T, Error>
where
    T: Clone + for<'de> Deserialize<'de> + Mytrait + Debug,
{
    let item2 = T::self_default();
    println!("{:#?}", item2);
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
                    Err(e) => {
                        messages.push(json!({
                            "message":
                                format!(
                                    "{} is not of required type {}, it is {}",
                                    value_key, rule, e
                                )
                        }));
                    }
                    _ => {}
                }
            }
        }
    }
    if messages.len() == 0 {
        let stuff = to_string(&item.into_inner())
            .map_err(|e| Error::from(e).to_response())
            .unwrap();
        let data: T = serde_json::from_str(&stuff)
            .map_err(|err| Error::from(err).to_response())
            .unwrap();
        return Ok(data);
    } else {
        return Err(Error::from(json!({ "messages": messages })));
    }
}

fn checktype(item: &Value, rule: &str) -> Result<(), String> {
    let value_type = match item {
        Value::Number(_) => "int",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
        Value::Bool(_) => "bool",
        _ => "not a value",
    };

    if rule != value_type {
        return Err(value_type.to_string());
    };
    Ok(())
}

trait TypeInfo {
    fn type_of(&self) -> &'static str;
}

impl TypeInfo for i32 {
    fn type_of(&self) -> &'static str {
        "i32"
    }
}

impl TypeInfo for i64 {
    fn type_of(&self) -> &'static str {
        "i64"
    }
}

impl TypeInfo for String {
    fn type_of(&self) -> &'static str {
        "String"
    }
}
