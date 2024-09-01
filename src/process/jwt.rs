use anyhow::Ok;
use std::{collections::BTreeMap, path::Path};

use chrono::{DateTime, Duration, Local};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

use hmac::{Hmac, Mac};

use crate::reader_from_input_u8;

pub struct JWTSign {
    key: Hmac<Sha256>,
}
impl JWTSign {
    pub fn new(key: Hmac<Sha256>) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(key)?;
        // let key = key.try_into()?;
        let signer = JWTSign::new(key);
        Ok(signer)
    }
}

pub struct JWTVerify {
    key: Hmac<Sha256>,
}
impl JWTVerify {
    pub fn new(key: Hmac<Sha256>) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(key)?;
        let signer = JWTVerify::new(key);
        Ok(signer)
    }
}

pub trait LoadKey {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

impl LoadKey for JWTSign {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = reader_from_input_u8(path.as_ref().to_str().unwrap());
        // let key = std::fs::read(path)?;
        let signer = JWTSign::try_new(&key.unwrap())?;
        Ok(signer)
    }
}

impl LoadKey for JWTVerify {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = reader_from_input_u8(path.as_ref().to_str().unwrap());
        // let key = std::fs::read(path)?;
        let verifier = JWTVerify::try_new(&key.unwrap())?;
        Ok(verifier)
    }
}

pub fn process_jwt_sign(sub: &str, aud: &str, key: &str, exp: &str) -> anyhow::Result<String> {
    let signer = JWTSign::load(key)?;
    let mut claims = BTreeMap::new();
    claims.insert("sub", sub);
    claims.insert("aud", aud);
    let mut datatime: DateTime<Local> = Local::now();
    if exp.ends_with("d") {
        let exp = exp.strip_suffix("d"); //&exp[..exp.len() - 1];
        datatime += Duration::days(exp.and_then(|s| s.parse::<i64>().ok()).unwrap());
    } else if exp.ends_with("m") {
        let exp = exp.strip_suffix("d");
        datatime += Duration::minutes(exp.and_then(|s| s.parse::<i64>().ok()).unwrap());
    } else if exp.ends_with("M") {
        let exp = exp.strip_suffix("d");
        datatime += Duration::weeks(exp.and_then(|s| s.parse::<i64>().ok()).unwrap());
    }
    let datatime_s = datatime.timestamp().to_string().clone();
    claims.insert("exp", datatime_s.as_str());
    // let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret")?;
    let token_str = claims.sign_with_key(&signer.key)?;
    // let token = signer.sign(input.as_bytes());
    // std::fs::write(output, token)?;
    Ok(token_str)
}

pub fn process_jwt_verify(input: &str, key: &str) -> anyhow::Result<BTreeMap<String, String>> {
    let verifier = JWTVerify::load(key)?;
    let claims: BTreeMap<String, String> = input.verify_with_key(&verifier.key)?;
    Ok(claims)
}
