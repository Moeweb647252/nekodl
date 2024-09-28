use crate::api::*;
use salvo::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReqData {
    id: usize
}