use anyhow::{Context, Result};
use rand::Rng;
use salvo::{async_trait, prelude::*};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::api::ApiResponse;

pub fn sha256(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:X}", hasher.finalize())
}

pub fn rand_str(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let possible_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut random_string = String::new();

    for _ in 0..length {
        let index = rng.gen_range(0..possible_chars.len());
        let char = possible_chars.chars().nth(index).unwrap();
        random_string.push(char);
    }

    random_string
}

#[async_trait]
impl<T> Writer for ApiResponse<T>
where
    T: Serialize + Send,
{
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.body(match serde_json::to_string(&self) {
            Ok(data) => data,
            Err(err) => err.to_string(),
        });
    }
}

pub trait FromDepot {
    fn from_depot(depot: &Depot) -> anyhow::Result<&Arc<RwLock<Self>>>;
}

impl<T> FromDepot for T
where
    T: Sync + Send + 'static,
{
    fn from_depot(depot: &Depot) -> anyhow::Result<&Arc<RwLock<Self>>> {
        Ok(depot.obtain::<Arc<RwLock<Self>>>().ok().context(format!(
            "Internal Error: file: {},lLine: {}",
            file!(),
            line!()
        ))?)
    }
}
#[cfg(test)]
mod test {
    use crate::utils::{rand_str, sha256};

    #[test]
    fn test_sha256() {
        println!("{}", sha256("aaaaa"));
    }

    #[test]
    fn test_rand_str() {
        println!("{}", rand_str(8));
    }

    #[test]
    fn test_rand_str_sha256() {
        println!("{}", sha256(&rand_str(8)));
    }
}
