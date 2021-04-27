use crate::common::helpers::AppResult;
use crate::config::CONFIG;
use crate::database::stores::mongo::mongo_index_builder::{
    sync_indexes, CollectionConfig, IndexOption, Indexes, MongoIndex,
};
use mongodb::Database;

pub async fn index_account_collection(db: &Database) -> AppResult<()> {
    // index accounts collection
    let index = Indexes::new()
        .with(
            MongoIndex::new("username")
                .with_option(IndexOption::Unique)
                .with_option(IndexOption::Sparse),
        )
        .with(
            MongoIndex::new("email")
                .with_option(IndexOption::Unique)
                .with_option(IndexOption::Sparse),
        )
        .with(
            MongoIndex::new("mobile")
                .with_option(IndexOption::Unique)
                .with_option(IndexOption::Sparse),
        );

    // println!("Indexing mongo database");
    sync_indexes(
        db,
        CollectionConfig {
            collection_name: "accounts",
            indexes: index,
        },
    )
    .await
}

pub async fn index_onetime_collection(db: &Database) -> AppResult<()> {
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

    // println!("Indexing mongo database");
    sync_indexes(
        db,
        CollectionConfig {
            collection_name: "one_time_codes",
            indexes: index,
        },
    )
    .await
}
