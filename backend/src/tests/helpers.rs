pub mod tests {
    use crate::config::CONFIG;
    use crate::database::connect::init_store;
    use crate::database::stores::base_store_trait::BoxedStoreType;
    use crate::features::appstate::state::{new_state, AppState};
    use actix_web::web::Data;
    use futures::executor::block_on;

    /// Returns a r2d2 Pooled Connection to be used in tests
    pub fn get_pool() -> BoxedStoreType {
        init_store(CONFIG.clone()).unwrap()
    }

    #[cfg(feature = "server-actix")]
    /// Returns a r2d2 Pooled Connection wrapped in Actix Application Data
    pub fn get_data_pool() -> Data<BoxedStoreType> {
        Data::new(get_pool())
    }

    // Mock application state
    pub fn app_state() -> AppState<'static, String> {
        new_state::<String>()
    }
}
