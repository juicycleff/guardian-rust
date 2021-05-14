use actix_guardian_identity::Identity;

use crate::common::auth::account::IdentityAccount;
use crate::common::auth::utils::hash;
use crate::common::errors::ApiError;
use crate::common::helpers::AppResult;
use crate::data::dtos::account_dto::AccountResponse;
use crate::data::dtos::auth_dto::*;
use crate::data::stores::base_store_trait::{BoxedStoreType, CreateAccountCommand};

/// The service handles creating a new account
pub async fn create_account(
    store: &BoxedStoreType,
    cmd: &PostAccountRequest,
) -> AppResult<AccountResponse> {
    let mut mobile: Option<String> = None;
    match &cmd.mobile {
        None => {}
        Some(m) => {
            mobile = Option::from(format!("{}-{}", m.prefix, m.digit));
        }
    }

    // lets make sure some auth is provided
    if cmd.email.is_none() && cmd.username.is_none() && cmd.mobile.is_none() {
        let err_messages = vec!["email, mobile or username field must be provided".to_string()];
        return Err(ApiError::ValidationError(err_messages));
    }

    let new_account = CreateAccountCommand {
        username: cmd.username.clone(),
        mobile,
        email: cmd.email.clone(),
        password: hash(cmd.password.as_str()),
    };

    let account = store.account_create(new_account).await?;
    let _ = store.onetime_code_create(account.id.as_str()).await;

    Ok(account.into())
}

/// The service handles finding accounts
pub async fn find_account(
    store: &BoxedStoreType,
    cmd: &IdentifierRequest,
) -> AppResult<AccountResponse> {
    let account = store
        .account_find_by_identity(cmd.identity.as_str())
        .await?;

    Ok(account.into())
}

/// The service handles deleting accounts
pub async fn delete_account(
    store: &BoxedStoreType,
    identity: &Identity,
    current_account: IdentityAccount,
) -> AppResult<bool> {
    let id = current_account.id;
    let rsp = store.account_delete(id.as_str(), false).await?;
    identity.forget();
    Ok(rsp)
}

/// The service handles locking accounts
pub async fn lock_account(store: &BoxedStoreType, id: String) -> AppResult<bool> {
    let rsp = store.account_lock(id.as_str()).await?;
    Ok(rsp)
}

/// The service handles unlocking accounts
pub async fn unlock_account(store: &BoxedStoreType, id: String) -> AppResult<bool> {
    let rsp = store.account_unlock(id.as_str()).await?;
    Ok(rsp)
}

#[cfg(test)]
mod test {
    use std::future::Future;
    use std::pin::Pin;

    use crate::common::tests::helpers::tests::get_store;
    use crate::data::dtos::account_dto::AccountResponse;
    use crate::data::dtos::auth_dto::*;

    use super::*;

    pub async fn create_account_test(cmd: PostAccountRequest) -> AppResult<AccountResponse> {
        create_account(&get_store(), &cmd).await
    }

    async fn seed_account_test() {
        let mut futures: Vec<Pin<Box<dyn Future<Output = AppResult<AccountResponse>>>>> = vec![];

        let test_users: Vec<PostAccountRequest> = vec![PostAccountRequest {
            email: Some("user_a@example.com".to_string()),
            password: "password".to_string(),
            confirm_password: "".to_string(),
            username: Some("user_a".parse().unwrap()),
            mobile: None,
        }];

        for ts in test_users.iter() {
            futures.push(Box::pin(create_account_test(ts.clone())));
        }

        let _ = futures::future::join_all(futures).await;
    }
}
