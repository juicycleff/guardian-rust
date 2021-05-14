pub mod tests {
    use crate::common::appstate::state::{new_state, AppState};
    use crate::config::CONFIG;
    use crate::data::connect::init_store;
    use crate::data::stores::base_store_trait::BoxedStoreType;
    use actix_web::web::Data;

    #[allow(unused)]
    pub fn get_store() -> BoxedStoreType {
        init_store(CONFIG.clone()).expect("could not get store for test")
    }

    #[cfg(feature = "server-actix")]
    #[allow(unused)]
    /// Returns a r2d2 Pooled Connection wrapped in Actix Application Data
    pub fn get_data_store() -> Data<BoxedStoreType> {
        Data::new(get_store())
    }

    // Mock application state
    #[allow(unused)]
    pub fn app_state() -> AppState<'static, String> {
        new_state::<String>()
    }
}
