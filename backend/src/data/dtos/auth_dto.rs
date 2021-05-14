use lazy_static::lazy_static;
use regex::Regex;
use validator::Validate;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").unwrap();
    static ref PHONE_NUMBER_REGEX: Regex =
        Regex::new(r"^[+]*[(]{0,1}[0-9]{1,4}[)]{0,1}[-\s\./0-9]*$").unwrap();
}

#[derive(juniper::GraphQLInputObject, Clone, Deserialize, Serialize, Validate, Debug)]
pub struct MobileInput {
    #[validate(
        length(min = 1, message = "mobile prefix too short"),
        regex(path = "PHONE_NUMBER_REGEX", message = "invalid prefix")
    )]
    pub prefix: String,

    #[validate(length(min = 1, message = "mobile prefix too short"))]
    pub digit: String,
}

#[derive(juniper::GraphQLInputObject, Clone, Deserialize, Serialize, Validate, Debug)]
pub struct PostSessionRequest {
    #[validate(length(
        min = 2,
        message = "Identity is required and must be at least 2 characters"
    ))]
    pub identity: String,

    #[validate(
        length(min = 1),
        custom = "crate::common::utils::validators::validate_strong_password"
    )]
    pub password: String,
}

#[derive(juniper::GraphQLInputObject, Clone, Debug, Deserialize, Serialize, Validate)]
pub struct PostAccountRequest {
    #[validate(
        email,
        length(min = 1, message = "Email is missing"),
        regex(path = "EMAIL_REGEX", message = "Invalid email address")
    )]
    pub email: Option<String>,

    #[validate(
        length(min = 1),
        custom = "crate::common::utils::validators::validate_strong_password"
    )]
    pub password: String,

    #[validate(must_match(other = "password", message = "Passwords does not match"))]
    pub confirm_password: String,

    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: Option<String>,

    pub mobile: Option<MobileInput>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct IdentifierRequest {
    #[validate(length(
        min = 3,
        message = "auth is required which is your username, mobile or email and must be at least 3 characters "
    ))]
    pub identity: String,
}
