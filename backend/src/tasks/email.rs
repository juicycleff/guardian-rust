use crate::data::models::accounts_model::AccountModel;
use celery::prelude::*;

/// task for processing emails
///
#[celery::task(max_retries = 3)]
pub async fn task_process_email(_acct: AccountModel) -> TaskResult<()> {
    println!("*******************");
    Ok(())
}
