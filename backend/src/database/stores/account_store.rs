use crate::common::errors::ApiError;
use crate::config::DatastoreConfig;
use crate::database::stores::base_store_trait::BaseStoreTrait;

#[cfg(feature = "sqlite")]
pub fn new_account_store(
    config: DatastoreConfig,
) -> Result<crate::database::stores::mysql::account_store::AccountStore, ApiError> {
    crate::database::stores::mysql::account_store::AccountStore::connect(config)
}

#[cfg(feature = "my-sql")]
pub fn new_account_store(
    config: DatastoreConfig,
) -> Result<crate::database::stores::mysql::account_store::AccountStore, ApiError> {
    crate::database::stores::mysql::account_store::AccountStore::connect(config)
}

#[cfg(feature = "postgres")]
pub fn new_account_store(
    config: DatastoreConfig,
) -> Result<crate::database::stores::postgres::account_store::AccountStore, ApiError> {
    crate::database::stores::postgres::account_store::AccountStore::connect(config)
}

#[cfg(feature = "mongo")]
pub fn new_account_store(
    config: DatastoreConfig,
) -> Result<crate::database::stores::mongo::account_store::AccountStore, ApiError> {
    crate::database::stores::mongo::account_store::AccountStore::connect(config)
}

#[cfg(feature = "cockroach")]
pub fn new_account_store(
    config: DatastoreConfig,
) -> Result<crate::database::stores::cockroach::account_store::AccountStore, ApiError> {
    crate::database::stores::cockroach::account_store::AccountStore::connect(config)
}
