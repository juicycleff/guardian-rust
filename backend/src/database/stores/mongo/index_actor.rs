use mongodb::{bson::doc, Database};
use riker::actors::{Actor, Context, Sender};
use slog::{info, warn};
use tokio::runtime;

use crate::common::helpers::AppResult;
use crate::common::utils::logger_utils::LOGGER;
use crate::config::CONFIG;
use crate::database::stores::mongo::mongo_index_builder::{
    sync_indexes, CollectionConfig, IndexOption, Indexes, MongoIndex,
};

async fn index_account_collection(db: &Database) -> AppResult<()> {
    // index accounts collection
    let index = Indexes::new()
        .with(
            MongoIndex::new("username")
                .with_option(IndexOption::Unique)
                .with_option(IndexOption::PartialFilterExpression(
                    doc! { "username": { "$type": "string" } },
                )),
        )
        .with(
            MongoIndex::new("email")
                .with_option(IndexOption::Unique)
                .with_option(IndexOption::PartialFilterExpression(
                    doc! { "email": { "$type": "string" } },
                )),
        )
        .with(
            MongoIndex::new("mobile")
                .with_option(IndexOption::Unique)
                .with_option(IndexOption::PartialFilterExpression(
                    doc! { "mobile": { "$type": "string" } },
                )),
        );

    let s = sync_indexes(
        db,
        CollectionConfig {
            collection_name: "accounts",
            indexes: index,
        },
    )
    .await;

    match s {
        Ok(_) => {
            info!(LOGGER, "[indexing] accounts collection indexed");
            Ok(())
        }
        Err(e) => {
            warn!(LOGGER, "[indexing] accounts collection indexing failed");
            Err(e)
        }
    }
}

async fn index_onetime_collection(db: &Database) -> AppResult<()> {
    // index accounts collection
    let index = Indexes::new()
        .with(
            MongoIndex::new("code")
                .with_key("account_id")
                .with_option(IndexOption::Unique),
        )
        .with(
            MongoIndex::new("expire_at").with_option(IndexOption::ExpireAfterSeconds(
                CONFIG.security.onetime_code_duration,
            )),
        );

    let s = sync_indexes(
        db,
        CollectionConfig {
            collection_name: "one_time_codes",
            indexes: index,
        },
    )
    .await;

    match s {
        Ok(_) => {
            info!(LOGGER, "[indexing] one_time_codes collection indexed");
            Ok(())
        }
        Err(e) => {
            warn!(
                LOGGER,
                "[indexing] one_time_codes collection indexing failed"
            );
            Err(e)
        }
    }
}

pub async fn index_db(db: &Database) -> AppResult<()> {
    let _ = tokio::try_join!(index_account_collection(&db), index_onetime_collection(&db));
    Ok(())
}

#[derive(Default)]
pub struct IndexMongoActor;

impl Actor for IndexMongoActor {
    type Msg = Database;

    fn recv(&mut self, _ctx: &Context<Database>, db: Database, _sender: Sender) {
        let rt = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let _ = rt.block_on(index_db(&db));
    }
}
