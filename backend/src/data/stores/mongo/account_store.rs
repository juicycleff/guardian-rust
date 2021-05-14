use std::time::Duration as StdDuration;

use async_trait::async_trait;
use chrono::{Duration, SecondsFormat, Utc};
use futures::executor::block_on;
use mongodb::{
    bson,
    bson::{doc, Document},
    options::ClientOptions,
    Client, Collection, Database,
};
use riker::actors::{ActorRefFactory, Timer};

use crate::common::errors::ApiError;
use crate::common::helpers::AppResult;
use crate::common::utils::ver_code_gen::gen_totp_code;
use crate::config::{DatastoreConfig, CONFIG};
use crate::data::models::accounts_model::AccountModel;
use crate::data::models::onetime_code_model::OneTimeCodeModel;
use crate::data::stores::base_store_trait::{
    BaseStoreTrait, CreateAccountCommand, TableNames, UpdateAccountCommand,
};
use crate::data::stores::mongo::index_actor;
use crate::data::stores::mongo::index_actor::IndexMongoActor;
use crate::events::SYSTEM;

fn get_id_query(id: &str) -> Document {
    let oid = mongodb::bson::oid::ObjectId::with_string(id).unwrap();
    doc! { "_id": oid, "delete_flag": false }
}

fn get_id_query_without_delete(id: &str) -> Document {
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

        match resp {
            None => Err(ApiError::NotFound("account not found".to_string())),
            Some(d) => {
                let serialized_resp = bson::from_document::<AccountModel>(d)?;
                Ok(serialized_resp)
            }
        }
    }

    async fn _update_one_account(&self, query: Document, payload: Document) -> AppResult<bool> {
        let account_col = &self._get_collection(TableNames::Accounts);
        let rsp = account_col.update_one(query, payload, None).await;
        match rsp {
            Ok(_) => Ok(true),
            Err(_) => Err(ApiError::DatabaseError(
                "there was a problem updating the account".to_string(),
            )),
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

        let store = AccountStore {
            client,
            config,
            db: db.clone(),
        };

        let index_actor_rsp = SYSTEM.actor_of::<IndexMongoActor>("my-actor");
        match index_actor_rsp {
            Ok(x_actor) => {
                let delay = StdDuration::from_secs(1);
                SYSTEM.schedule_once(delay, x_actor, None, db);
            }
            Err(_) => {}
        }

        Result::Ok(store)
    }

    fn ping(&self) -> Result<(), ApiError> {
        todo!()
    }

    async fn index_db(&self) -> AppResult<()> {
        index_actor::index_db(&self.db).await
    }

    async fn account_create(&self, cmd: CreateAccountCommand) -> AppResult<AccountModel> {
        let account_col = &self._get_collection(TableNames::Accounts);

        let email = match cmd.email {
            None => bson::Bson::Null,
            Some(s) => bson::Bson::String(s),
        };

        let username = match cmd.username {
            None => bson::Bson::Null,
            Some(s) => bson::Bson::String(s),
        };

        let mobile = match cmd.mobile {
            None => bson::Bson::Null,
            Some(s) => bson::Bson::String(s),
        };

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

        let result = account_col.insert_one(doc_data, None).await;

        match result {
            Ok(resp) => {
                let id = bson::from_bson::<bson::oid::ObjectId>(resp.inserted_id)?;
                self.account_find_by_id(&id.to_hex()).await
            }
            Err(_err) => Err(ApiError::Conflict(
                "account with auth not available".to_string(),
            )),
        }
    }

    async fn account_update(&self, id: &str, cmd: UpdateAccountCommand) -> AppResult<AccountModel> {
        let query = get_id_query(&id);

        let payload = mongodb::bson::to_bson::<UpdateAccountCommand>(&cmd)?;
        let update_payload = doc! {
            "$set": payload,
            "$currentDate": { "updated_at": true }
        };

        // update record
        let updated = self
            ._update_one_account(query.clone(), update_payload)
            .await?;

        if !updated {
            let err = "could not updated account".to_string();
            Err(ApiError::DatabaseError(err))
        } else {
            // return updated record
            self._find_one_account(query).await
        }
    }
    async fn account_find_by_id(&self, id: &str) -> AppResult<AccountModel> {
        let filter = get_id_query(&id);
        self._find_one_account(filter).await
    }

    async fn account_find_by_username(&self, username: &str) -> AppResult<AccountModel> {
        let filter = doc! { "username": username, "delete_flag": false };
        self._find_one_account(filter).await
    }

    async fn account_find_by_email(&self, email: &str) -> AppResult<AccountModel> {
        let filter = doc! { "email": email, "delete_flag": false };
        self._find_one_account(filter).await
    }

    async fn account_find_by_mobile(&self, mobile: &str) -> AppResult<AccountModel> {
        let filter = doc! { "mobile": mobile, "delete_flag": false };
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
        let acct = self.account_find_by_id(&id).await?;
        if acct.locked {
            return Err(ApiError::Conflict(
                "the account is already locked".to_string(),
            ));
        }

        let query = get_id_query(&id);
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

        let update_payload = doc! { "$set": {
                "locked": true,
                "updated_at": now,
            },
        };

        self._update_one_account(query, update_payload).await
    }

    async fn account_unlock(&self, id: &str) -> AppResult<bool> {
        let acct = self.account_find_by_id(&id).await?;
        if !acct.locked {
            return Err(ApiError::Conflict("the account is not locked".to_string()));
        }

        let query = get_id_query(&id);
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

        let update_payload = doc! { "$set": {
                "locked": false,
                "updated_at": now,
            },
        };

        self._update_one_account(query, update_payload).await
    }

    async fn account_require_new_password(&self, id: &str) -> AppResult<bool> {
        let _ = self.account_find_by_id(&id).await?;
        let query = get_id_query(&id);
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

        let update_payload = doc! { "$set": {
                "locked": true,
                "updated_at": now,
            },
        };

        self._update_one_account(query, update_payload).await
    }

    async fn account_set_password(&self, id: &str, password: &str) -> AppResult<bool> {
        let _ = self.account_find_by_id(&id).await?;
        let query = get_id_query(&id);
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

        let update_payload = doc! { "$set": {
                "updated_at": now.clone(),
                "password": password,
                "require_new_password": false,
                "password_changed_at": now,
            },
        };

        self._update_one_account(query, update_payload).await
    }

    async fn account_set_last_login(&self, id: &str) -> AppResult<bool> {
        let _ = self.account_find_by_id(&id).await?;
        let query = get_id_query(&id);
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

        let update_payload = doc! { "$set": {
                "last_login_at": now,
            },
        };

        self._update_one_account(query, update_payload).await
    }

    async fn account_delete(&self, id: &str, hard_delete: bool) -> AppResult<bool> {
        let filter = get_id_query_without_delete(&id);
        let resp = self._find_one_account(filter).await;

        if resp.is_err() {
            return Err(ApiError::NotFound(
                "account by id does not exist".to_string(),
            ));
        }

        if hard_delete {
            let account_col = &self._get_collection(TableNames::Accounts);
            let query = get_id_query_without_delete(&id);
            let _ = account_col.delete_one(query, None).await?;
        }

        let query = get_id_query_without_delete(&id);
        let update_payload = doc! { "$set":
            {
                "delete_flag": true,
            },
        };
        self._update_one_account(query, update_payload).await
    }

    async fn onetime_code_create(&self, account_id: &str) -> AppResult<OneTimeCodeModel> {
        let otp_col = &self._get_collection(TableNames::OneTimeCodes);

        // first let try to get an existing otp that's not expired
        let doc_rsp = self.onetime_code_find_by_account(account_id, None).await;

        // return existing code or skip to create
        if let Ok(otp) = doc_rsp {
            return Result::Ok(otp);
        }

        // create new otp
        let new_date = Utc::now();
        let expire_at = new_date
            .checked_add_signed(Duration::seconds(
                CONFIG.security.onetime_code_duration as i64,
            ))
            .unwrap_or_else(Utc::now);
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let code = gen_totp_code(account_id.to_string());

        let doc_data = doc! {
            "code": code,
            "expire_at": expire_at,
            "created_by": account_id,
            "created_at": now.clone(),
            "updated_by": now,
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

    async fn onetime_code_find_by_account(
        &self,
        account_id: &str,
        code: Option<&str>,
    ) -> AppResult<OneTimeCodeModel> {
        let otp_col = &self._get_collection(TableNames::OneTimeCodes);

        let filter = match code {
            Some(c) => doc! { "created_by": account_id, "code": c },
            None => doc! { "created_by": account_id },
        };

        let doc_rsp = otp_col.find_one(filter, None).await?;

        match doc_rsp {
            None => Err(ApiError::DatabaseError(
                "no onetime code for this account".to_string(),
            )),
            Some(docz) => Ok(bson::from_document::<OneTimeCodeModel>(docz)?),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DatastoreConfig;
    use crate::data::stores::base_store_trait::TableNames;

    async fn get_db() -> AccountStore {
        let cfg = DatastoreConfig {
            db_url: "mongodb://localhost:27017".to_string(),
            db_name: "guardian_test".to_string(),
            redis_url: "localhost:6379".to_string(),
        };

        let store_res = AccountStore::connect(cfg);
        assert_eq!(store_res.is_err(), false);

        let store = store_res.unwrap();

        // create collections
        let _ = store
            .db
            .create_collection(TableNames::Accounts.to_string().as_str(), None)
            .await;
        let _ = store
            .db
            .create_collection(TableNames::OneTimeCodes.to_string().as_str(), None)
            .await;

        // index db
        let _ = store.index_db().await;

        store
    }

    async fn seed_db(store: &AccountStore) -> AppResult<AccountModel> {
        store
            .account_create(CreateAccountCommand {
                password: "password".to_string(),
                email: Some("test@test.com".to_string()),
                username: Some("test".to_string()),
                mobile: Some("35674677".to_string()),
            })
            .await
    }

    async fn remove_doc(store: &AccountStore, id: String) {
        let _ = store.account_delete(id.as_str(), false).await.unwrap();
    }

    #[actix_rt::test]
    async fn it_can_create_account() {
        let store = get_db().await;
        let acct = store
            .account_create(CreateAccountCommand {
                password: "password".to_string(),
                email: Some("test@test.com".to_string()),
                username: Some("test".to_string()),
                mobile: Some("35674677".to_string()),
            })
            .await
            .unwrap();

        assert_eq!(acct.username.unwrap(), "test".to_string());
        let _ = store.account_delete(acct.id.as_str(), true).await;
    }

    #[actix_rt::test]
    async fn it_can_create_account_with_only_username() {
        let store = get_db().await;

        let first_acct = store
            .account_create(CreateAccountCommand {
                password: "password".to_string(),
                email: None,
                username: Some("tester".to_string()),
                mobile: None,
            })
            .await
            .unwrap();
        assert_eq!(first_acct.username.unwrap(), "tester".to_string());

        let second_acct = store
            .account_create(CreateAccountCommand {
                password: "password".to_string(),
                email: None,
                username: Some("test2".to_string()),
                mobile: None,
            })
            .await
            .unwrap();
        assert_eq!(second_acct.username.unwrap(), "test2".to_string());

        let _ = store.account_delete(first_acct.id.as_str(), true).await;
        let _ = store.account_delete(second_acct.id.as_str(), true).await;
    }

    #[actix_rt::test]
    async fn it_cannot_create_account() {
        let store = get_db().await;

        // create first account
        let acct = store
            .account_create(CreateAccountCommand {
                password: "password".to_string(),
                email: Some("test9@test.com".to_string()),
                username: Some("test9".to_string()),
                mobile: Some("35674679".to_string()),
            })
            .await
            .unwrap();

        let result = store
            .account_create(CreateAccountCommand {
                password: "password".to_string(),
                email: Some("test9@test.com".to_string()),
                username: Some("test9".to_string()),
                mobile: Some("35674679".to_string()),
            })
            .await
            .unwrap_err();
        let expect = ApiError::Conflict("account with auth not available".to_string());
        assert_eq!(result, expect);
        let _ = store.account_delete(acct.id.as_str(), true).await;
    }

    #[actix_rt::test]
    async fn it_can_soft_delete_account() {
        let store = get_db().await;

        // create first account
        let acct = store
            .account_create(CreateAccountCommand {
                password: "password".to_string(),
                email: None,
                username: Some("delete_user2".to_string()),
                mobile: None,
            })
            .await
            .unwrap();
        let del_rsp = store.account_delete(acct.id.as_str(), false).await;
        assert_eq!(del_rsp.is_err(), false);

        let result = store
            .account_find_by_id(acct.id.as_str())
            .await
            .unwrap_err();
        let expect = ApiError::NotFound("account not found".to_string());
        assert_eq!(result, expect);
        let _ = store.account_delete(acct.id.as_str(), true).await;
    }

    #[actix_rt::test]
    async fn it_can_hard_delete_account() {
        let store = get_db().await;

        // create first account
        let acct = store
            .account_create(CreateAccountCommand {
                password: "password".to_string(),
                email: None,
                username: Some("delete_user2".to_string()),
                mobile: None,
            })
            .await
            .unwrap();
        let del_rsp = store.account_delete(acct.id.as_str(), true).await;
        assert_eq!(del_rsp.is_err(), false);

        let result = store
            .account_find_by_id(acct.id.as_str())
            .await
            .unwrap_err();
        let expect = ApiError::NotFound("account not found".to_string());
        assert_eq!(result, expect);
    }

    #[actix_rt::test]
    async fn it_can_lock_account() {
        let store = get_db().await;

        let acct = seed_db(&store).await.unwrap();
        assert_eq!(acct.locked, false);

        let bool_resp = store.account_lock(acct.id.as_str()).await.unwrap();
        assert_eq!(bool_resp, true);

        let result = store.account_find_by_id(acct.id.as_str()).await.unwrap();
        assert_eq!(result.locked, true);
        let _ = store.account_delete(acct.id.as_str(), true).await;
    }

    #[actix_rt::test]
    async fn it_cannot_lock_a_locked_account() {
        let store = get_db().await;

        let acct = seed_db(&store).await.unwrap();
        assert_eq!(acct.locked, false);

        let bool_resp = store.account_lock(acct.id.as_str()).await.unwrap();
        assert_eq!(bool_resp, true);

        let result = store.account_lock(acct.id.as_str()).await.unwrap_err();
        let expect = ApiError::Conflict("the account is already locked".to_string());
        assert_eq!(result, expect);

        let _ = store.account_delete(acct.id.as_str(), true).await;
    }
}
