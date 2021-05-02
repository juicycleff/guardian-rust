use async_trait::async_trait;

use crate::common::errors::ApiError;
use crate::common::helpers::AppResult;
use crate::config::DatastoreConfig;
use crate::database::models::accounts_model::AccountModel;
use crate::database::models::onetime_code_model::OneTimeCodeModel;

#[derive(strum_macros::ToString, Debug)]
pub enum TableNames {
    #[strum(serialize = "accounts")]
    Accounts,
    #[strum(serialize = "one_time_codes")]
    OneTimeCodes,
}

#[derive(Debug)]
pub struct CreateAccountCommand {
    pub password: String,
    pub email: Option<String>,
    pub username: Option<String>,
    pub mobile: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountCommand {
    pub email: Option<String>,
    pub username: Option<String>,
    pub mobile: Option<String>,
}

#[async_trait]
/// A generic store trait implemented by all store type
pub trait BaseStoreTrait {
    /// Create a new account
    ///
    /// Returns Self or BoxedStoreType
    ///
    /// # Arguments
    ///
    /// * `config` - {DatastoreConfig} The account store configuration or options
    ///
    /// # Examples
    /// use crate::database::stores::base_store_trait::BoxedStoreType;
    /// let opts = DatastoreConfig {}
    /// let store = BoxedStoreType::connect(opts).await?;
    fn connect(config: DatastoreConfig) -> Result<Self, ApiError>
    where
        Self: Sized;

    fn ping(&self) -> Result<(), ApiError>;

    async fn index_db(&self) -> AppResult<()>;

    /// Create a new account
    ///
    /// Returns the account if found or throws an error
    ///
    /// # Arguments
    ///
    /// * `id` - An account ID
    /// * `cmd` - An account update payload UpdateAccountCommand
    ///
    /// # Examples
    /// use crate::database::stores::base_store_trait::BoxedStoreType;
    /// let opts = CreateAccountCommand {
    ///     email: Option::from("example@example.com"),
    ///     username: Option::from("example),
    ///     mobile: None,
    /// }
    /// let account = BoxedStoreType::account_create("id", opts).await?;
    async fn account_create(&self, cmd: CreateAccountCommand) -> AppResult<AccountModel>;

    /// Update an existing account detail
    ///
    /// Returns the account if found or throws an error
    ///
    /// # Arguments
    ///
    /// * `id` - An account ID
    /// * `cmd` - An account update payload UpdateAccountCommand
    ///
    /// # Examples
    /// use crate::database::stores::base_store_trait::BoxedStoreType;
    /// let opts = UpdateAccountCommand {
    ///     email: Option::from("example@example.com"),
    ///     username: Option::from("example),
    ///     mobile: None,
    /// }
    /// let account = BoxedStoreType::account_update("id", opts).await?;
    async fn account_update(&self, id: &str, cmd: UpdateAccountCommand) -> AppResult<AccountModel>;

    /// Find an account by primary key or id
    ///
    /// Returns the account if found or throws an error
    ///
    /// # Arguments
    ///
    /// * `id` - An account ID
    ///
    /// # Examples
    /// use crate::database::stores::base_store_trait::BoxedStoreType;
    ///
    /// let account = BoxedStoreType::account_find_by_id("username").await?;
    async fn account_find_by_id(&self, id: &str) -> AppResult<AccountModel>;

    /// Find an account by username
    ///
    /// Returns the account if found or throws an error
    ///
    /// # Arguments
    ///
    /// * `username` - A account username
    ///
    /// # Examples
    /// use crate::database::stores::base_store_trait::BoxedStoreType;
    ///
    /// let account = BoxedStoreType::account_find_by_username("username").await?;
    async fn account_find_by_username(&self, username: &str) -> AppResult<AccountModel>;

    /// Find an account by email
    ///
    /// Returns the account if found or throws an error
    ///
    /// # Arguments
    ///
    /// * `email` - A account email
    ///
    /// # Examples
    /// use crate::database::stores::base_store_trait::BoxedStoreType;
    ///
    /// let account = BoxedStoreType::account_find_by_email("username").await?;
    async fn account_find_by_email(&self, email: &str) -> AppResult<AccountModel>;

    /// Find an account by mobile
    ///
    /// Returns the account if found or throws an error
    ///
    /// # Arguments
    ///
    /// * `mobile` - An account mobile
    ///
    /// # Examples
    /// use crate::database::stores::base_store_trait::BoxedStoreType;
    ///
    /// let account = BoxedStoreType::account_find_by_mobile("username").await?;
    async fn account_find_by_mobile(&self, mobile: &str) -> AppResult<AccountModel>;

    /// Find an account by email, mobile or username
    ///
    /// Returns the account if found or throws an error
    ///
    /// # Arguments
    ///
    /// * `identity` - A account username, email or mobile
    ///
    /// # Examples
    /// use crate::database::stores::base_store_trait::BoxedStoreType;
    ///
    /// let account = BoxedStoreType::account_find_by_identity("username").await?;
    async fn account_find_by_identity(&self, identity: &str) -> AppResult<AccountModel>;

    /// Lock an account. This stops an account from logging in
    ///
    /// Returns boolean
    ///
    /// # Arguments
    ///
    /// * `id` - An account ID
    ///
    /// # Examples
    /// use crate::database::stores::base_store_trait::BoxedStoreType;
    ///
    /// let account = BoxedStoreType::account_lock("username").await?;
    async fn account_lock(&self, id: &str) -> AppResult<bool>;

    /// Un-Lock an account.
    ///
    /// Returns boolean
    ///
    /// # Arguments
    ///
    /// * `id` - An account ID
    ///
    /// # Examples
    /// use crate::database::stores::base_store_trait::BoxedStoreType;
    ///
    /// let state = BoxedStoreType::account_un_lock("username").await?;
    async fn account_un_lock(&self, id: &str) -> AppResult<bool>;

    /// Marks a account for requiring password change
    ///
    /// Returns boolean
    ///
    /// # Arguments
    ///
    /// * `id` - An account ID
    ///
    /// # Examples
    /// use crate::database::stores::base_store_trait::BoxedStoreType;
    ///
    /// let state = BoxedStoreType::account_require_new_password("username").await?;
    async fn account_require_new_password(&self, id: &str) -> AppResult<bool>;

    async fn account_set_password(&self, id: &str, password: &str) -> AppResult<bool>;

    async fn account_set_last_login(&self, id: &str) -> AppResult<bool>;

    async fn account_delete(&self, id: &str, hard_delete: bool) -> AppResult<bool>;

    async fn onetime_code_create(&self, id: &str) -> AppResult<OneTimeCodeModel>;

    async fn onetime_code_find_by_account(
        &self,
        account_id: &str,
        code: Option<&str>,
    ) -> AppResult<OneTimeCodeModel>;
}
pub type BoxedStoreType = Box<dyn BaseStoreTrait + Send + Sync>;
