use serde_json::Value;

#[derive(Clone, Debug)]
pub struct Request {
    pub query: Value,
    pub form_data: Value,
}
