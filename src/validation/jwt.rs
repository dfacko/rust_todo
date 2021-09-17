use crate::models::models::*;
use chrono::prelude::*;
use uuid::Uuid;
use jsonwebtoken;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    pass: String,
    id: String,
    exp: i64,
}

pub fn generate(user: User) -> String {
    let secret = String::from("MyJwtSecret");
    let duration = 1200;
    let exp = Utc::now() + chrono::Duration::seconds(duration);

    let claims = Claims {
        sub: String::from(user.username),
        pass: String::from(user.pword),
        id: user.id.to_string(),
        exp: exp.timestamp(),
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(&secret.as_bytes()),
    )
    .unwrap_or_default()
}

pub fn verify(token: String) -> Result<User, jsonwebtoken::errors::Error> {
    let secret = String::from("MyJwtSecret");

    let data = jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;

    Ok(User {
        id: Uuid::parse_str(&data.claims.id).unwrap(),
        username: String::from(data.claims.sub),
        pword: String::from(data.claims.pass),
    })
}
