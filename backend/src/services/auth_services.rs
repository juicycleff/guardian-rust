use crate::common::auth::utils::hash;
use crate::common::errors::ApiError;
use crate::common::helpers::AppResult;
use crate::database::stores::base_store_trait::{BoxedStoreType, CreateAccountCommand};
use crate::dtos::account_dto::AccountResponse;
use crate::dtos::auth_dto::*;

/// This service handles authenticating an identity
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
        Some(ref e) => match account.unconfirmed_email {
            None => {}
            Some(ref v) => {
                if *v == *e {
                    let err_message = "please verify your account".to_string();
                    return Err(ApiError::Unauthorized(err_message));
                }
            }
        }
    }

    Ok(account.into())
}

/// The service handles creating a new account
pub async fn create_account(
    store: &BoxedStoreType,
    cmd: &PostAccountRequest,
) -> Result<AccountResponse, ApiError> {
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
    let _ = store.onetime_code_create(account.id.as_str()).await?;

    Ok(account.into())
}

#[cfg(test)]
mod test {
    use chrono::Utc;

    use crate::common::errors::ApiError;
    use crate::database::models::accounts_model::UpdateAccount;
    use crate::tests::helpers::tests::get_pool;

    use super::*;

    pub fn create_account(username: &str, email: &str, pass: &str) -> Result<Account, ApiError> {
        let new_account = NewAccount {
            username: Option::from(username.to_string()),
            mobile: None,
            email: email.to_string(),
            password: pass.to_string(),
            locked_at: None,
            confirmation_sent_at: None,
            confirmation_token: None,
            locked: Option::from(false),
        };

        let acct: Account = new_account.into();
        create(&get_pool(), &acct)
    }

    fn seed_account() -> Result<Account, ApiError> {
        create_account("user_a", "user_a@example.com", "password")
    }
}
