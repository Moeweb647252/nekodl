use crate::api::*;
use salvo::prelude::*;

#[handler]
pub async fn status(req: &mut Request, depot: &mut Depot) {}
