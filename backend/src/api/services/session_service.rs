use crate::common::auth::utils::hash;
use crate::common::errors::ApiError;
use crate::common::helpers::AppResult;
use crate::data::dtos::account_dto::AccountResponse;
use crate::data::dtos::auth_dto::PostSessionRequest;
use crate::data::stores::base_store_trait::BoxedStoreType;
use actix_guardian_identity::Identity;

/// This service handles authenticating an auth
pub async fn create_session(
    store: &BoxedStoreType,
    cmd: &PostSessionRequest,
) -> AppResult<AccountResponse> {
    let account = store
        .account_find_by_identity(cmd.identity.as_str())
        .await?;

    let hashed_password = hash(&cmd.password);
    if account.password != hashed_password {
        let err_message = "your email, username or password is incorrect".to_string();
        println!("error with password");
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

/// This service handles deleting session or logging out.
pub async fn delete_session(_store: &BoxedStoreType, identity: &Identity) -> AppResult<bool> {
    identity.forget();

    Ok(true)
}

#[cfg(test)]
mod test {
    use super::*;
}
