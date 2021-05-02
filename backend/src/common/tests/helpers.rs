pub mod tests {
    use crate::config::CONFIG;
    use crate::database::connect::init_store;
    use crate::database::stores::base_store_trait::BoxedStoreType;
    use crate::features::appstate::state::{new_state, AppState};
    use actix_web::web::Data;

    /// Returns a r2d2 Pooled Connection to be used in tests
    pub fn get_store() -> BoxedStoreType {
        init_store(CONFIG.clone()).expect("could not get store for test")
    }

    #[cfg(feature = "server-actix")]
    /// Returns a r2d2 Pooled Connection wrapped in Actix Application Data
    pub fn get_data_store() -> Data<BoxedStoreType> {
        Data::new(get_store())
    }

    // Mock application state
    pub fn app_state() -> AppState<'static, String> {
        new_state::<String>()
    }
}
