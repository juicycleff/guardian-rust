#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentityAccount {
    pub id: String,
    pub email: Option<String>,
    pub username: Option<String>,
    pub mobile: Option<String>,
}
