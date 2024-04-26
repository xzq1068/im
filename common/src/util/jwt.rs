use anyhow::anyhow;
use base64::{ Engine};
use base64::prelude::BASE64_URL_SAFE;
use chrono::Utc;
use jsonwebtoken::{DecodingKey, encode, EncodingKey, get_current_timestamp, Header, Validation};

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    /// Optional. Audience
    aud: i64,
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
pub fn audience_of_token(token: &str) -> Result<i64> {
     let payload = match token.split(".").nth(1) {
        None => {return Err(anyhow!("not payload"))}
        Some(payload) => payload,
    };

    let res = BASE64_URL_SAFE.decode(payload)?;

    let claims = serde_json::from_slice::<Claims>(res.as_slice())?;

    Ok(claims.aud)
}

#[inline]
pub async fn verify_token(token: &str, key: &[u8], audience: i64) -> anyhow::Result<()> {
    let res = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key),
        &Validation::default(),
    )?;
    if res.claims.aud != audience {
        return Err(anyhow!("invalid token"));
    }
    if res.claims.exp < Utc::now().timestamp() as u64 {
        return Err(anyhow!("token expired"));
    }
    if res.claims.iss != "XZQ".to_string() {
        return Err(anyhow!("invalid token"));
    }

    Ok(())
}

#[inline]
pub fn simple_token(key:&[u8],audience:i64) -> String {
    let t = get_current_timestamp();

    encode(
        &Header::default(),
        &Claims {
            aud: audience,
            exp: t + 7 * 24 * 60 * 60,
            iat: t,
            iss: "PRIM".to_string(),
            nbf: t,
            sub: "".to_string(),
        },
        &EncodingKey::from_secret(key),
    ).unwrap()


}