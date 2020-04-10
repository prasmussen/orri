use std::time::{SystemTime, UNIX_EPOCH};
use ring::digest;
use hex;
use rand::{Rng};
use rand::distributions;


pub fn unix_timestamp(time: SystemTime) -> u64 {
    time
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}


pub fn ensure<E>(value: bool, err: E) -> Result<(), E> {
    if value {
        Ok(())
    } else {
        Err(err)
    }
}


pub fn sha256(bytes: &[u8]) -> String {
    let digest = digest::digest(&digest::SHA256, bytes);
    hex::encode(digest.as_ref())
}
