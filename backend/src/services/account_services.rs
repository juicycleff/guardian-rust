use crate::common::auth::utils::hash;
use crate::common::errors::ApiError;
use crate::common::helpers::AppResult;
use crate::database::stores::base_store_trait::{BoxedStoreType, CreateAccountCommand};
use crate::dtos::account_dto::AccountResponse;
use crate::dtos::auth_dto::*;

/// This service handles authenticating an identity
pub async fn login_account(
    store: &BoxedStoreType,
    cmd: &PostSessionRequest,
) -> AppResult<AccountResponse> {
    let account = store
        .account_find_by_identity(cmd.identity.as_str())
        .await?;

    let hashed_password = hash(&cmd.password);
    if account.password != hashed_password {
        let err_message = "your email, username or password is incorrect".to_string();
        println!("error with paswword");
        return Err(ApiError::Unauthorized(err_message));
    }

    if account.locked {
        let err_message = "your account has been locked out, please contact support".to_string();
        return Err(ApiError::Unauthorized(err_message));
    }

    if account.require_new_password {
        let err_message = "please request a new password change".to_string();
        return Err(ApiError::Unauthorized(err_message));
    }

    match account.email {
        None => {}
        Some(ref email) => match &account.unconfirmed_email {
            None => {}
            Some(ref v) => {
                if *v == *email {
                    let err_message = "Please verify your account".to_string();
                    return Err(ApiError::Unauthorized(err_message));
                }
            }
        },
    }

    Ok(account.into())
}

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

    // lets make sure some identity is provided
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

#[cfg(test)]
mod test {
    use crate::common::tests::helpers::tests::get_store;
    use crate::dtos::account_dto::AccountResponse;
    use crate::dtos::auth_dto::*;

    use super::*;
    use std::future::Future;
    use std::pin::Pin;

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
