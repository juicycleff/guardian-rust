use crate::common::auth::account::IdentityAccount;
use crate::common::auth::utils::{decode_jwt, PrivateClaim};
use actix_guardian_identity::RequestIdentity;
use actix_web::{
    dev::Payload,
    web::{HttpRequest, HttpResponse},
    Error, FromRequest,
};
use futures::future::{err, ok, Ready};

/// Extractor for pulling the auth out of a request.
///
/// Simply add "user: IdentityAccount" to a handler to invoke this.
impl FromRequest for IdentityAccount {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // req.headers().get()
        let identity = RequestIdentity::get_identity(req);

        if let Some(identity) = identity {
            let private_claim: PrivateClaim = decode_jwt(&identity).unwrap();
            return ok(IdentityAccount {
                id: private_claim.sub.to_string(),
                email: private_claim.email,
                mobile: private_claim.mobile,
                username: private_claim.username,
            });
        }
        err(HttpResponse::Unauthorized().into())
    }
}
