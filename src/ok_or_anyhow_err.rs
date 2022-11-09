use anyhow::{anyhow, Result};
use std::error::Error;

pub(crate) fn ok_or_anyhow_err<T>(
    result: std::result::Result<T, Box<dyn Error>>,
    msg: &str,
) -> Result<T> {
    match result {
        Ok(ok) => Ok(ok),
        Err(err) => Err(anyhow!("{}:\n\n{}", msg, err)),
    }
}
