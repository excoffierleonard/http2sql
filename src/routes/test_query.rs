use crate::{db::DbPool, errors::ApiError};
use actix_web::{
    get,
    web::{Data, Json},
    HttpResponse, Responder, Result,
};
use serde::{Deserialize, Serialize};

#[get("/v1/custom")]
async fn custom_query(
    pool: Data<DbPool>,
    query: Json<CustomQueryRequest>,
) -> Result<impl Responder, ApiError> {
}
