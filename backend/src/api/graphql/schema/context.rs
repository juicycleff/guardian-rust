use crate::database::stores::base_store_trait::BoxedStoreType;
use actix_identity::Identity;
use actix_web::web::Data;

#[derive(Clone)]
pub struct Context {
    pub store: Data<BoxedStoreType>,
    // pub identity: Mutex<Identity>,
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

impl Context {
    pub fn new(store: Data<BoxedStoreType>, _identity: Identity) -> Self {
        Context {
            store: store.clone(),
            // identity: Mutex::new(identity),
        }
    }
}
