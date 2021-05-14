use actix_web::web::{Data, Json};

use crate::api::services;
use crate::common::auth::account::IdentityAccount;
use crate::common::auth::utils::{create_jwt, PrivateClaim};
use crate::common::helpers::{respond_json, AppResult};
use crate::common::validate::validate;
use crate::data::dtos::account_dto::PostAccountResponse;
use crate::data::dtos::auth_dto::*;
use crate::data::stores::base_store_trait::BoxedStoreType;
use actix_guardian_identity::Identity;

/// Handler for creating a new account
pub async fn post_account(
    store: Data<BoxedStoreType>,
    body: Json<PostAccountRequest>,
) -> AppResult<Json<PostAccountResponse>> {
    validate(&body)?;

    let cmd: PostAccountRequest = body.into_inner();
    let acct = services::account_service::create_account(&store, &cmd).await?;

    // create jwt token
    let pc = PrivateClaim::new(acct.id, acct.email, acct.username, acct.mobile);
    let token = create_jwt(pc).unwrap_or_default();

    respond_json(PostAccountResponse {
        id_token: Some(token),
    })
}

/// Handler for deleting accounts
pub async fn delete_account(
    store: Data<BoxedStoreType>,
    id: Identity,
    current_account: IdentityAccount,
) -> AppResult<Json<bool>> {
    let acct = services::account_service::delete_account(&store, &id, current_account).await?;
    respond_json(acct)
}

/// Handler for unlocking accounts
pub async fn unlock_account(
    store: Data<BoxedStoreType>,
    current_account: IdentityAccount,
) -> AppResult<Json<bool>> {
    let id = current_account.id;
    let acct = services::account_service::unlock_account(&store, id).await?;
    respond_json(acct)
}

/// Handler for locking accounts
pub async fn lock_account(
    store: Data<BoxedStoreType>,
    current_account: IdentityAccount,
) -> AppResult<Json<bool>> {
    let id = current_account.id;
    let acct = services::account_service::lock_account(&store, id).await?;
    respond_json(acct)
}

/// This handler checks if an account is available or not.
pub async fn get_available_account(
    store: Data<BoxedStoreType>,
    body: Json<IdentifierRequest>,
) -> AppResult<Json<bool>> {
    validate(&body)?;

    let cmd: IdentifierRequest = body.into_inner();
    let acct = services::account_service::find_account(&store, &cmd).await;

    match acct {
        Ok(_) => respond_json(true),
        Err(_) => respond_json(false),
    }
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
