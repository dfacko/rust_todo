use super::models::*;
use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    pass: String,
    id: i32,
    exp: i64,
}

pub fn generate(user: User) -> String {
    let secret = String::from("MyJwtSecret");
    let duration = 1200;
    let exp = Utc::now() + chrono::Duration::seconds(duration);

    let claims = Claims {
        sub: String::from(user.username),
        pass: String::from(user.pword),
        id: user.id,
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
        id: data.claims.id,
        username: String::from(data.claims.sub),
        pword: String::from(data.claims.pass),
    })
}
