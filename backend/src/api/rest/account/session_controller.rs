use actix_guardian_identity::Identity;
use actix_web::web::{Data, Json};

use crate::api::services;
use crate::common::auth::utils::{create_jwt, PrivateClaim};
use crate::common::helpers::{respond_json, AppResult};
use crate::common::validate::validate;
use crate::data::dtos::account_dto::PostAccountResponse;
use crate::data::dtos::auth_dto::PostSessionRequest;
use crate::data::stores::base_store_trait::BoxedStoreType;

/// Handler to login a user and create a session for the user
pub async fn post_session(
    id: Identity,
    store: Data<BoxedStoreType>,
    body: Json<PostSessionRequest>,
) -> AppResult<Json<PostAccountResponse>> {
    validate(&body)?;

    let cmd: PostSessionRequest = body.into_inner();
    let acct = services::session_service::create_session(&store, &cmd).await?;

    // create jwt token
    let pc = PrivateClaim::new(acct.id, acct.email, acct.username, acct.mobile);
    let token = create_jwt(pc)?;
    id.remember(token.clone());

    respond_json(PostAccountResponse {
        id_token: Some(token),
    })
}

/// Handler to delete account from store
pub async fn delete_session(
    identity: Identity,
    store: Data<BoxedStoreType>,
) -> AppResult<Json<bool>> {
    let resp = services::session_service::delete_session(&store, &identity).await?;
    respond_json(resp)
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
