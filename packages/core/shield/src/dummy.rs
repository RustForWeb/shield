use async_trait::async_trait;

use crate::{
    error::ShieldError,
    form::Form,
    provider::{Provider, Subprovider},
    request::{SignInCallbackRequest, SignInRequest, SignOutRequest},
    response::Response,
    session::Session,
    storage::Storage,
};

pub const DUMMY_PROVIDER_ID: &str = "dummy";

#[derive(Clone, Default)]
pub struct DummyProvider {
    subprovider: DummySubprovider,
}

impl DummyProvider {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Provider for DummyProvider {
    fn id(&self) -> String {
        DUMMY_PROVIDER_ID.to_owned()
    }

    async fn subproviders(&self) -> Result<Vec<Box<dyn Subprovider>>, ShieldError> {
        Ok(vec![Box::new(self.subprovider.clone())])
    }

    async fn subprovider_by_id(
        &self,
        _subprovider_id: &str,
    ) -> Result<Option<Box<dyn Subprovider>>, ShieldError> {
        Ok(None)
    }

    async fn sign_in(
        &self,
        _request: SignInRequest,
        _session: Session,
    ) -> Result<Response, ShieldError> {
        todo!("redirect back?")
    }

    async fn sign_in_callback(
        &self,
        _request: SignInCallbackRequest,
        _session: Session,
    ) -> Result<Response, ShieldError> {
        todo!("redirect back?")
    }

    async fn sign_out(
        &self,
        _request: SignOutRequest,
        _session: Session,
    ) -> Result<Response, ShieldError> {
        todo!("redirect back?")
    }
}

#[derive(Clone, Default)]
pub struct DummySubprovider {}

impl Subprovider for DummySubprovider {
    fn provider_id(&self) -> String {
        DUMMY_STORAGE_ID.to_owned()
    }

    fn id(&self) -> Option<String> {
        None
    }

    fn name(&self) -> String {
        "Dummy".to_owned()
    }

    fn form(&self) -> Option<Form> {
        None
    }
}

pub const DUMMY_STORAGE_ID: &str = "dummy";

#[derive(Clone, Default)]
pub struct DummyStorage {}

impl DummyStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Storage for DummyStorage {
    fn id(&self) -> String {
        DUMMY_STORAGE_ID.to_owned()
    }
}
