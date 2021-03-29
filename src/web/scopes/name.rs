use actix_web::{
    dev::Payload,
    Error,
    FromRequest,
    HttpRequest,
};
use futures::future::{ok, Ready};

#[derive(Clone)]
pub struct NameScope {
    pub name: String,
}

impl FromRequest for NameScope {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let name = req.match_info().get("name").unwrap().to_string();

        ok(Self { name })
    }
}
