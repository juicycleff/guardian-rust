use std::time::Duration as StdDuration;

use async_trait::async_trait;
use chrono::{Duration, Utc, SecondsFormat};
use futures::executor::{block_on};
use futures::try_join;
use mongodb::{
    bson,
    bson::{doc, Document},
    Client,
    Collection, Database, options::ClientOptions,
};

use crate::common::errors::ApiError;
use crate::common::helpers::AppResult;
use crate::config::{CONFIG, DatastoreConfig};
use crate::database::models::accounts_model::AccountModel;
use crate::database::models::onetime_code_model::OneTimeCodeModel;
use crate::database::stores::base_store_trait::{
    BaseStoreTrait, CreateAccountCommand, TableNames, UpdateAccountCommand,
};
use crate::database::stores::mongo::mongo_index_builder::{
    CollectionConfig, Indexes, IndexOption, MongoIndex, sync_indexes,
};
use crate::utils::ver_code_gen::verification_code_gen;
use std::thread;
use crate::database::stores::mongo::index_actor::{IndexActor, IndexMongo};
use actix::Actor;

fn get_id_query(id: &str) -> Document {
    let oid = mongodb::bson::oid::ObjectId::with_string(id).unwrap();
    doc! { "_id": oid }
}

#[derive(Clone)]
pub struct AccountStore {
    client: Client,
    config: DatastoreConfig,
    db: Database,
}

impl AccountStore {

    async fn _find_one_account(&self, filter: Document) -> AppResult<AccountModel> {
        let account_col = &self._get_collection(TableNames::Accounts);
        let resp = account_col.find_one(filter, None).await?;
        let serialized_resp = bson::from_document::<AccountModel>(resp.expect("account not found"))?;
        Ok(serialized_resp)
    }

    async fn _update_one_account(&self, query: Document, payload: Document) -> AppResult<bool> {
        let account_col = &self._get_collection(TableNames::Accounts);
        let rsp = account_col.update_one(query, payload, None).await;
        match rsp {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn _get_collection(&self, col: TableNames) -> Collection<Document> {
        self.db.collection::<Document>(col.to_string().as_str())
    }
}

#[async_trait]
impl BaseStoreTrait for AccountStore {
    fn connect(config: DatastoreConfig) -> AppResult<Self> {
        let mut client_options = block_on(ClientOptions::parse("mongodb://localhost:27017"))?;
        client_options.app_name = Some(config.clone().db_name);
        let client = Client::with_options(client_options)?;
        let db = client.database(&config.db_name);

        let store = AccountStore { client, config, db: db.clone() };
        // let _ = block_on(store.clone().index_db());

        // Start MyActor in current thread
        let addr = IndexActor.start();
        let _ = addr.do_send(IndexMongo { db: db.clone() });

        Result::Ok(store)
    }

    fn ping(&self) -> Result<(), ApiError> {
        todo!()
    }

    async fn index_db(&self) -> AppResult<()> {
        todo!()
    }

    async fn account_create(&self, cmd: CreateAccountCommand) -> AppResult<AccountModel> {
        let account_col = &self._get_collection(TableNames::Accounts);

        let email = &cmd.email.unwrap_or_default();
        let username = &cmd.username.unwrap_or_default();
        let mobile = &cmd.mobile.unwrap_or_default();
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

        let doc_data = doc! {
            "email": email,
            "username": username,
            "mobile": mobile,
            "locked": false,
            "require_new_password": false,
            "password_changed_at": bson::Bson::Null,
            "created_at": &now,
            "updated_at": now,
            "password": cmd.password,
            "delete_flag": false,
            "enable_2fa": false,
        };

        let resp = account_col.insert_one(doc_data, None).await?;
        let id = bson::from_bson::<bson::oid::ObjectId>(resp.inserted_id)?;
        self.account_find_by_id(&id.to_hex()).await
    }

    async fn account_update(&self, id: &str, cmd: UpdateAccountCommand) -> AppResult<AccountModel> {
        todo!()
    }
    async fn account_find_by_id(&self, id: &str) -> AppResult<AccountModel> {
        let filter = get_id_query(&id);
        self._find_one_account(filter).await
    }

    async fn account_find_by_username(&self, username: &str) -> AppResult<AccountModel> {
        let filter = doc! { "username": username };
        self._find_one_account(filter).await
    }

    async fn account_find_by_email(&self, email: &str) -> AppResult<AccountModel> {
        let filter = doc! { "email": email };
        self._find_one_account(filter).await
    }

    async fn account_find_by_mobile(&self, mobile: &str) -> AppResult<AccountModel> {
        let filter = doc! { "mobile": mobile };
        self._find_one_account(filter).await
    }

    async fn account_find_by_identity(&self, identity: &str) -> AppResult<AccountModel> {
        let filter = doc! { "$or": [
            {
                "mobile": &identity,
            },
            {
                "username": &identity,
            },
            {
                "email": &identity,
            },
        ]};
        self._find_one_account(filter).await
    }

    async fn account_lock(&self, id: &str) -> AppResult<bool> {
        let _ = self.account_find_by_id(&id).await?;
        let query = get_id_query(&id);
        let updated_at = Utc::now();

        let update_payload = doc! { "$set": [
            {
                "locked": true,
                "updated_at": updated_at,
            },
        ]};

        self._update_one_account(query, update_payload).await
    }

    async fn account_un_lock(&self, id: &str) -> AppResult<bool> {
        let _ = self.account_find_by_id(&id).await?;
        let query = get_id_query(&id);
        let updated_at = Utc::now();

        let update_payload = doc! { "$set": [
            {
                "locked": false,
                "updated_at": updated_at,
            },
        ]};

        self._update_one_account(query, update_payload).await
    }

    async fn account_require_new_password(&self, id: &str) -> AppResult<bool> {
        let _ = self.account_find_by_id(&id).await?;
        let query = get_id_query(&id);
        let updated_at = Utc::now();

        let update_payload = doc! { "$set": [
            {
                "locked": true,
                "updated_at": updated_at,
            },
        ]};

        self._update_one_account(query, update_payload).await
    }

    async fn account_set_password(&self, id: &str, password: &str) -> AppResult<bool> {
        let _ = self.account_find_by_id(&id).await?;
        let query = get_id_query(&id);
        let now = Utc::now();

        let update_payload = doc! { "$set": [
            {
                "updated_at": now,
                "password": password,
                "require_new_password": false,
                "password_changed_at": now,
            },
        ]};

        self._update_one_account(query, update_payload).await
    }

    async fn account_set_last_login(&self, id: &str) -> AppResult<bool> {
        let _ = self.account_find_by_id(&id).await?;
        let query = get_id_query(&id);
        let last_login_at = Utc::now();

        let update_payload = doc! { "$set": [
            {
                "last_login_at": last_login_at,
            },
        ]};

        self._update_one_account(query, update_payload).await
    }

    async fn onetime_code_create(&self, account_id: &str) -> AppResult<OneTimeCodeModel> {
        let otp_col = &self._get_collection(TableNames::OneTimeCodes);

        // first let try to get an existing otp that's not expired
        let doc_rsp = self.onetime_code_find_by_account(account_id, None).await;

        // return existing code or skip to create
        if let Ok(otp) = doc_rsp { return Result::Ok(otp) }

        // create new otp
        let new_date = Utc::now();
        let expire_at = new_date
            .checked_add_signed(Duration::seconds(CONFIG.security.onetime_code_duration as i64))
            .unwrap_or_else(Utc::now);
        let now = Utc::now();
        let code = verification_code_gen(CONFIG.security.onetime_code_length);
        let doc_data = doc! {
            "code": code,
            "expire_at": expire_at,
            "created_by": account_id,
            "created_at": bson::Bson::DateTime(now),
            "updated_by": bson::Bson::DateTime(now),
            "updated_at": bson::Bson::Null,
            "deleted_at": bson::Bson::Null,
            "delete_flag": false,
        };

        let resp = otp_col.insert_one(doc_data, None).await?;
        let id = bson::from_bson::<bson::oid::ObjectId>(resp.inserted_id)?;

        // get created otp
        let filter = get_id_query(&id.to_hex());
        let doc_resp = otp_col.find_one(filter, None).await?;
        let otp = bson::from_document::<OneTimeCodeModel>(doc_resp.unwrap())?;
        Ok(otp)
    }

    async fn onetime_code_find_by_account(&self, account_id: &str, code: Option<&str>) -> AppResult<OneTimeCodeModel> {
        let otp_col = &self._get_collection(TableNames::OneTimeCodes);

        let filter = match code {
            Some(c) => doc! { "created_by": account_id, "code": c },
            None => doc! { "created_by": account_id }
        };

        let doc_rsp = otp_col.find_one(filter, None).await?;

        match doc_rsp {
            None => Err(ApiError::DatabaseError("no onetime code for this account".to_string())),
            Some(docz) => Ok(bson::from_document::<OneTimeCodeModel>(docz)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn get_db() -> AccountStore {
        let cfg = DatastoreConfig {
            db_url: "mongodb://localhost:27017".to_string(),
            db_name: "guardian_test".to_string(),
            redis_url: "".to_string(),
        };

        let store = AccountStore::connect(cfg).unwrap();
        store.index_db().await;
        store
    }

    #[actix_rt::test]
    async fn it_can_create_account() {
        let store = get_db().await;

        let acct = store
            .account_create(CreateAccountCommand {
                password: "my-password".to_string(),
                email: Some("example@example.com".to_string()),
                username: Some("example".to_string()),
                mobile: Some("35674677".to_string()),
            })
            .await
            .unwrap();

        assert_eq!(acct.username.unwrap(), "example@example.coms".to_string());
    }
}
