use crate::database::models::accounts_model::AccountModel;
use chrono::{DateTime, Utc};
use rayon::prelude::*;

#[graphql(description = "A humanoid creature in the Star Wars universe")]
#[derive(juniper::GraphQLObject, Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountResponse {
    pub id: String,
    pub username: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[graphql(description = "A humanoid creature in the Star Wars universe")]
#[derive(juniper::GraphQLObject, Debug, Deserialize, Serialize, PartialEq)]
pub struct PostSessionResponse {
    pub id_token: Option<String>,
}

#[graphql(description = "A humanoid creature in the Star Wars universe")]
#[derive(juniper::GraphQLObject, Debug, Deserialize, Serialize, PartialEq)]
pub struct PostAccountResponse {
    pub id_token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountsResponse(pub Vec<AccountResponse>);

impl From<AccountModel> for AccountResponse {
    fn from(acct: AccountModel) -> Self {
        AccountResponse {
            id: acct.id.parse().unwrap(),
            username: acct.username,
            mobile: acct.mobile,
            email: acct.email,
            created_at: acct.created_at,
            updated_at: acct.updated_at,
        }
    }
}

impl From<Vec<AccountModel>> for AccountsResponse {
    fn from(acct: Vec<AccountModel>) -> Self {
        AccountsResponse(acct.into_par_iter().map(|a| a.into()).collect())
    }
}
