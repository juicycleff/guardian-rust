//! Database-related functions
use actix_web::web;

use crate::common::helpers::AppResult;
use crate::config::{Config, CONFIG};
use crate::data::stores::account_store::new_account_store;
use crate::data::stores::base_store_trait::{BaseStoreTrait, BoxedStoreType};
use std::sync::Mutex;

pub fn init_store(config: Config) -> AppResult<BoxedStoreType> {
    let store = new_account_store(config.datastore).expect("Failed to create connection pool");
    let _ = store.index_db(); //.and_then(|_| println!("db indexed successfully"));
    Result::Ok(Box::from(store))
}

pub fn add_pool(cfg: &mut web::ServiceConfig) {
    let boxed_store = init_store(CONFIG.clone()).expect("Failed to create connection pool");
    cfg.data(boxed_store);
}

pub fn add_shared_state(cfg: &mut web::ServiceConfig) {
    let boxed_store = init_store(CONFIG.clone()).expect("Failed to create connection pool");
    let mut_boxed_store = web::Data::new(Mutex::new(boxed_store));

    cfg.app_data(mut_boxed_store.clone());
}
