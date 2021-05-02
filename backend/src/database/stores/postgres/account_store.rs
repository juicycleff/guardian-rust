use crate::common::errors::ApiError;
use crate::common::helpers::AppResult;
use crate::config::DatastoreConfig;
use crate::database::models::accounts_model::AccountModel;
use crate::database::models::onetime_code_model::OneTimeCodeModel;
use crate::database::stores::base_store_trait::{
    BaseStoreTrait, CreateAccountCommand, UpdateAccountCommand,
};
use async_trait::async_trait;

pub struct AccountStore {
    _client: String,
    config: DatastoreConfig,
}

#[async_trait]
impl BaseStoreTrait for AccountStore {
    fn connect(config: DatastoreConfig) -> Result<Self, ApiError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn ping(&self) -> Result<(), ApiError> {
        todo!()
    }

    async fn index_db(&self) -> AppResult<()> {
        todo!()
    }

    async fn account_create(&self, cmd: CreateAccountCommand) -> AppResult<AccountModel> {
        todo!()
    }

    async fn account_update(&self, id: &str, cmd: UpdateAccountCommand) -> AppResult<AccountModel> {
        todo!()
    }

    async fn account_find_by_id(&self, id: &str) -> AppResult<AccountModel> {
        todo!()
    }

    async fn account_find_by_username(&self, username: &str) -> AppResult<AccountModel> {
        todo!()
    }

    async fn account_find_by_email(&self, email: &str) -> AppResult<AccountModel> {
        todo!()
    }

    async fn account_find_by_mobile(&self, mobile: &str) -> AppResult<AccountModel> {
        todo!()
    }

    async fn account_find_by_identity(&self, identity: &str) -> AppResult<AccountModel> {
        todo!()
    }

    async fn account_lock(&self, id: &str) -> AppResult<bool> {
        todo!()
    }

    async fn account_un_lock(&self, id: &str) -> AppResult<bool> {
        todo!()
    }

    async fn account_require_new_password(&self, id: &str) -> AppResult<bool> {
        todo!()
    }

    async fn account_set_password(&self, id: &str, password: &str) -> AppResult<bool> {
        todo!()
    }

    async fn account_set_last_login(&self, id: &str) -> AppResult<bool> {
        todo!()
    }

    async fn account_delete(&self, id: &str, hard_delete: bool) -> AppResult<bool> {
        todo!()
    }

    async fn onetime_code_create(&self, id: &str) -> AppResult<OneTimeCodeModel> {
        todo!()
    }

    async fn onetime_code_find_by_account(
        &self,
        account_id: &str,
        code: Option<&str>,
    ) -> AppResult<OneTimeCodeModel> {
        todo!()
    }
}
