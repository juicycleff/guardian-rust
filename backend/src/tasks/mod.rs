use celery::export::Arc;
use celery::{broker::RedisBroker, Celery};
use once_cell::sync::OnceCell;

pub mod email;

pub static TASK_CLIENT: OnceCell<Arc<Celery<RedisBroker>>> = OnceCell::new();

pub fn task_client() -> &'static Arc<Celery<RedisBroker>> {
    TASK_CLIENT.get().expect("celery is not initialized")
}
