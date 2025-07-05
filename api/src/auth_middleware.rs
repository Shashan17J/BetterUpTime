use std::env;

use jsonwebtoken::{DecodingKey, Validation, decode};
use poem::{Error, FromRequest, Request, RequestBody, Result, http::StatusCode};
use dotenvy::dotenv;
use crate::routes::user::Claims;

pub struct UserId(pub String);

impl<'a> FromRequest<'a> for UserId {
    /// Extract from request head and body.
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| panic!("Please provide the jwt environment variable"));
        let token = req
            .headers()
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| Error::from_string("missing token", StatusCode::UNAUTHORIZED))?;

        let token_data = decode::<Claims>(
            &token,
            // &DecodingKey::from_secret("secret".as_ref()),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| Error::from_string("token malformed", StatusCode::UNAUTHORIZED))?;

        Ok(UserId(token_data.claims.sub))
    }
}
