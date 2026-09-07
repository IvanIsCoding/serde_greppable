mod gron;
mod ungron;

use std::io::{Read, Write};

use serde::{Serialize, de::DeserializeOwned};

pub use gron::json_to_gron;
pub use ungron::gron_to_json;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Gron(#[from] anyhow::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let value = serde_json::to_value(value)?;
    Ok(json_to_gron(&value))
}

pub fn to_writer<T, W>(value: &T, mut writer: W) -> Result<()>
where
    T: ?Sized + Serialize,
    W: Write,
{
    writer.write_all(to_string(value)?.as_bytes())?;
    Ok(())
}

pub fn from_str<T>(input: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    Ok(serde_json::from_value(gron_to_json(input)?)?)
}

pub fn from_slice<T>(input: &[u8]) -> Result<T>
where
    T: DeserializeOwned,
{
    from_str(std::str::from_utf8(input)?)
}

pub fn from_reader<T, R>(mut reader: R) -> Result<T>
where
    T: DeserializeOwned,
    R: Read,
{
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    from_str(&input)
}
