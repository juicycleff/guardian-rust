use crate::common::errors::ApiError;
use crate::config::DatastoreConfig;
use crate::data::stores::base_store_trait::BaseStoreTrait;

#[cfg(feature = "sqlite")]
pub fn new_account_store(
    config: DatastoreConfig,
) -> Result<crate::data::stores::mysql::account_store::AccountStore, ApiError> {
    crate::data::stores::mysql::account_store::AccountStore::connect(config)
}

#[cfg(feature = "my-sql")]
pub fn new_account_store(
    config: DatastoreConfig,
) -> Result<crate::data::stores::mysql::account_store::AccountStore, ApiError> {
    crate::data::stores::mysql::account_store::AccountStore::connect(config)
}

#[cfg(feature = "postgres")]
pub fn new_account_store(
    config: DatastoreConfig,
) -> Result<crate::data::stores::postgres::account_store::AccountStore, ApiError> {
    crate::data::stores::postgres::account_store::AccountStore::connect(config)
}

#[cfg(feature = "mongo")]
pub fn new_account_store(
    config: DatastoreConfig,
) -> Result<crate::data::stores::mongo::account_store::AccountStore, ApiError> {
    crate::data::stores::mongo::account_store::AccountStore::connect(config)
}

#[cfg(feature = "cockroach")]
pub fn new_account_store(
    config: DatastoreConfig,
) -> Result<crate::data::stores::cockroach::account_store::AccountStore, ApiError> {
    crate::data::stores::cockroach::account_store::AccountStore::connect(config)
}
