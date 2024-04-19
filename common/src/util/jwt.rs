use anyhow::anyhow;
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE;
use chrono::Utc;
use jsonwebtoken::{DecodingKey, Validation};

pub type Result<T> = anyhow::Result<T>;
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    /// Optional. Audience
    aud: u64,
    /// Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    exp: u64,
    /// Optional. Issued at (as UTC timestamp)
    iat: u64,
    /// Optional. Issuer
    iss: String,
    /// Optional. Not Before (as UTC timestamp)
    nbf: u64,
    /// Optional. Subject (whom token refers to)
    sub: String,
}

#[inline]
pub fn audience_of_token(token:&str) -> Result<u64> {
    let payload = token.split(".").nth(1).unwrap();

    let res = BASE64_URL_SAFE.decode(payload)?;

    let claims=serde_json::from_slice::<Claims>(res.as_slice())?;

    Ok(claims.aud)
}

#[inline]
pub async fn verify_token(token: &str, key: &[u8], audience: u64) -> anyhow::Result<()> {
    let res = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key),
        &Validation::default(),
    )?;
    if res.claims.aud!=audience {
        return Err(anyhow!("invalid token"));
    }
    if res.claims.exp < Utc::now().timestamp() as u64{
        return Err(anyhow!("token expired"));
    }
    if res.claims.iss!="XZQ".to_string() {
        return Err(anyhow!("invalid token"));
    }

    Ok(())
}
