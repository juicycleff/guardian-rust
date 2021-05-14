/*
use actix_identity::Identity;
use actix_protobuf::ProtoBuf;
use actix_protobuf::*;
use actix_web::web::Data;
use actix_web::HttpResponse;

use crate::common::auth::utils::{create_jwt, PrivateClaim};
use crate::common::errors::ApiError;
use crate::common::helpers::{respond_json, AppResult};
use crate::common::validate::validate;
use crate::data::stores::base_store_trait::BoxedStoreType;
use crate::dtos::account_dto::{AccountResponse, PostAccountResponse};
use crate::dtos::auth_dto::*;
use crate::services;

/// Handler to login a user and create a session for the user
pub async fn post_session_grpc(
    id: Identity,
    store: Data<BoxedStoreType>,
    msg: ProtoBuf<PostSessionRequest>,
) -> AppResult<HttpResponse<PostAccountResponse>> {
    validate(&msg)?;

    let cmd = PostSessionRequest {
        auth: msg.auth.clone(),
        password: msg.password.clone(),
    };
    let acct = services::account_services::login_account(&store, &cmd).await?;
    id.remember(acct.id.clone());

    // create jwt token
    let pc = PrivateClaim::new(acct.id, acct.email, acct.username, acct.mobile);
    let token = create_jwt(pc)?;

    HttpResponse::Ok().protobuf(PostAccountResponse {
        id_token: Some(token),
    })
}

/// Handler for creating a new account
pub async fn post_account_grpc(
    store: Data<BoxedStoreType>,
    msg: ProtoBuf<PostAccountRequest>,
) -> AppResult<HttpResponse<PostAccountResponse>> {
    validate(&msg)?;

    let cmd = PostAccountRequest {
        email: msg.email.clone(),
        password: msg.password.clone(),
        confirm_password: msg.confirm_password.clone(),
        username: msg.username.clone(),
        mobile: msg.mobile.clone(),
    };
    let acct = services::account_services::create_account(&store, &cmd).await?;

    // create jwt token
    let pc = PrivateClaim::new(acct.id, acct.email, acct.username, acct.mobile);
    let token = create_jwt(pc).unwrap_or_default();

    HttpResponse::Ok().protobuf(PostAccountResponse {
        id_token: Some(token),
    })
}

/// This handler checks if an account is available or not.
pub async fn get_available_account_grpc(
    id: Identity,
    store: Data<BoxedStoreType>,
    msg: ProtoBuf<IdentifierRequest>,
) -> AppResult<HttpResponse<AccountResponse>> {
    validate(&msg)?;

    println!("Helllo {}", id.auth().unwrap_or_default());
    let cmd = IdentifierRequest {
        auth: msg.auth.clone(),
    };
    let acct = services::account_services::find_account(&store, &cmd).await;

    match acct {
        Ok(_) => HttpResponse::Ok().protobuf(true),
        Err(_) => HttpResponse::Ok().protobuf(false),
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

 */
