use actix_identity::Identity;
use actix_web::web::{Data, Json};

use crate::common::errors::ApiError;
use crate::common::helpers::{respond_json, AppResult};
use crate::common::validate::validate;
use crate::database::stores::base_store_trait::BoxedStoreType;
use crate::dtos::account_dto::{AccountResponse, PostAccountResponse};
use crate::dtos::auth_dto::*;
use crate::services;

/// Handler to login a user and create a session for the user
pub async fn post_session(
    id: Identity,
    store: Data<BoxedStoreType>,
    body: Json<PostSessionRequest>,
) -> Result<Json<PostAccountResponse>, ApiError> {
    validate(&body)?;

    let cmd: PostSessionRequest = body.into_inner();
    let acct = services::account_services::login_account(&store, &cmd).await?;
    id.remember(acct.id);

    respond_json(PostAccountResponse {
        id_token: id.identity(),
    })
}

/// Handler for creating a new account
pub async fn post_account(
    store: Data<BoxedStoreType>,
    body: Json<PostAccountRequest>,
) -> Result<Json<PostAccountResponse>, ApiError> {
    validate(&body)?;

    let cmd: PostAccountRequest = body.into_inner();
    let _ = services::account_services::create_account(&store, &cmd).await?;

    respond_json(PostAccountResponse { id_token: None })
}

/// This handler checks if an account is available or not.
pub async fn get_available_account(
    store: Data<BoxedStoreType>,
    body: Json<IdentifierRequest>,
) -> AppResult<Json<AccountResponse>> {
    validate(&body)?;

    let cmd: IdentifierRequest = body.into_inner();
    let acct = services::account_services::find_account(&store, &cmd).await;

    /* match acct {
        Ok(_) => respond_json(true),
        Err(_) => respond_json(false),
    } */

    respond_json(acct.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_login_with_bad_request() {
        // let response = get_health().await.unwrap();
        // assert_eq!(response.into_inner().status, "ok".to_string());
    }
}
