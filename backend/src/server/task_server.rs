use celery::broker::RedisBroker;

use crate::common::helpers::AppResult;
use crate::config::CONFIG;
use crate::tasks::email::task_process_email;
use crate::tasks::TASK_CLIENT;

/// Task queue entry server
pub async fn start_task_server() -> AppResult<()> {
    let task_system = celery::app!(
        broker = RedisBroker { &CONFIG.datastore.redis_url },
        tasks = [task_process_email],

        task_routes = [
            "task_process_email" => "process-email",
            "*" => "celery",
        ],
        prefetch_count = 2,
        heartbeat = Some(10),
    )
    .await?;

    // set cell one
    let _ = TASK_CLIENT.set(task_system);

    Ok(())
}
