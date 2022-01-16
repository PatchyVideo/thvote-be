use jwt_simple::prelude::{ES256kPublicKey, ES256kKeyPair};


#[derive(Debug, Clone)]
pub struct Context {
    pub user_ip: String,
    pub additional_fingureprint: Option<String>,
    pub public_key: ES256kKeyPair
}

impl juniper::Context for Context {}
