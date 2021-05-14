use crate::data::stores::base_store_trait::BoxedStoreType;
use actix_web::web::Data;
use juniper::futures::lock::Mutex;

#[derive(Clone)]
pub struct Context {
    pub store: Data<BoxedStoreType>,
    // pub auth: IdentityAccount,
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

impl Context {
    pub fn new(store: Data<BoxedStoreType>) -> Self {
        Context {
            store: store.clone(),
        }
    }
}
