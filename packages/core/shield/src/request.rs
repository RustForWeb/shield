use serde_json::Value;
#[cfg(feature = "utoipa")]
use utoipa::openapi::HttpMethod;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Delete,
    Options,
    Head,
    Patch,
    Trace,
}

#[cfg(feature = "utoipa")]
impl From<RequestMethod> for HttpMethod {
    fn from(value: RequestMethod) -> Self {
        match value {
            RequestMethod::Get => Self::Get,
            RequestMethod::Post => Self::Post,
            RequestMethod::Put => Self::Put,
            RequestMethod::Delete => Self::Delete,
            RequestMethod::Options => Self::Options,
            RequestMethod::Head => Self::Head,
            RequestMethod::Patch => Self::Patch,
            RequestMethod::Trace => Self::Trace,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Request {
    pub query: Value,
    pub form_data: Value,
}
