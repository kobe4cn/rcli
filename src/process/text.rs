use std::{fs, io::Read, path::Path, str::FromStr};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{cli::GenPassOpts, reader_from_input, TextSignFormat};

use super::process_genpass;

//加密
pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let buffer = reader_from_input(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut buffer.as_bytes())?
        }
        TextSignFormat::ED25519 => {
            let signer = ED25519Signer::load(key)?;
            signer.sign(&mut buffer.as_bytes())?
        }
    };

    Ok(signed)
}

//验证
pub fn process_verify(
    input: &str,
    key: &str,
    signature: &str,
    format: TextSignFormat,
) -> anyhow::Result<bool> {
    let buffer = reader_from_input(input)?;

    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut buffer.as_bytes(), signature)?
        }
        TextSignFormat::ED25519 => {
            let verifier = ED25519Verifier::load(key)?;

            verifier.verify(&mut buffer.as_bytes(), signature)?
        }
    };

    Ok(verified)
}

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<String, anyhow::Error>;
}

pub trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, signature: &str) -> Result<bool, anyhow::Error>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self, anyhow::Error>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}
pub struct ED25519Signer {
    key: SigningKey,
}
pub struct ED25519Verifier {
    key: VerifyingKey,
}

impl TextSign for ED25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<String, anyhow::Error> {
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)?;
        let signature = self.key.sign(&buffer);

        Ok(signature.to_string())
    }
}
impl TextVerify for ED25519Verifier {
    fn verify(&self, reader: &mut dyn Read, signature: &str) -> Result<bool, anyhow::Error> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let signature = Signature::from_str(signature.trim())?;

        let ret = self.key.verify(&buffer, &signature).is_ok();

        Ok(ret)
    }
}
impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<String, anyhow::Error> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Ok(blake3::keyed_hash(&self.key, &buffer).to_string())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, signature: &str) -> Result<bool, anyhow::Error> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let hash = blake3::keyed_hash(&self.key, &buffer).to_string();
        Ok(hash == signature)
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self, anyhow::Error> {
        // let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self, anyhow::Error> {
        let key = fs::read(path)?;
        let key = Self::try_new(&key)?;
        Ok(key)
    }
}

impl KeyLoader for ED25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self, anyhow::Error> {
        let key = fs::read(path)?;
        let key = Self::try_new(&key)?;
        Ok(key)
    }
}

impl KeyLoader for ED25519Verifier {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        let key = Self::try_new(&key)?;
        Ok(key)
    }
}

impl ED25519Signer {
    pub fn new(key: [u8; 32]) -> Self {
        Self {
            key: SigningKey::from_bytes(&key),
        }
    }

    pub fn try_new(key: &[u8]) -> Result<Self, anyhow::Error> {
        // let key = &key[..32];
        let key = key.try_into()?;
        let signer = ED25519Signer::new(key);
        Ok(signer)
    }
}

impl ED25519Verifier {
    // pub fn new(key: [u8; 32]) -> Self {
    //     Self {
    //         key: VerifyingKey::from_bytes(&key),
    //     }
    // }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        // let key = &key[..32];
        let key = key.try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        // let verifier = ED25519Verifier::new(key);
        Ok(Self { key })
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let opts = GenPassOpts {
            length: 32,
            uppercase: true,
            lowercase: true,
            number: true,
            symbol: true,
        };
        let key = process_genpass(opts)?;
        Ok(vec![key.as_bytes().to_vec()])
    }
}

impl KeyGenerator for ED25519Signer {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let verifying_key: VerifyingKey = signing_key.verifying_key();
        Ok(vec![
            signing_key.to_bytes().to_vec(),
            verifying_key.to_bytes().to_vec(),
        ])
    }
}

pub fn process_key_generate(format: TextSignFormat) -> anyhow::Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::ED25519 => ED25519Signer::generate(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_verify() -> anyhow::Result<()> {
        let sk = ED25519Signer::load("fixture/ed25519.sk")?;
        let pk = ED25519Verifier::load("fixture/ed25519.pk")?;
        let data = b"hello world";
        let sig = sk.sign(&mut &data[..])?;
        assert!(pk.verify(&mut &data[..], &sig)?);
        Ok(())
    }
}
