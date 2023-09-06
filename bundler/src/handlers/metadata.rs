use actix_web::web::{Data, Json};

use crate::errors::ApiError;
use crate::models::admin::metadata_response::MetadataResponse;
use crate::models::response::base_response::BaseResponse;
use crate::provider::helpers::respond_json;
use crate::services::token_metadata_service::TokenMetadataService;

pub async fn get_metadata(
    service: Data<TokenMetadataService>,
) -> Result<Json<BaseResponse<MetadataResponse>>, ApiError> {
    let response = service.get_chain().await?;
    respond_json(response)
}
