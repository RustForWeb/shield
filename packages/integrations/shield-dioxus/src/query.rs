use std::{collections::HashMap, ops::Deref};

#[derive(Clone, Debug)]
pub struct Query(HashMap<String, String>);

impl Query {
    pub fn parse(query: &str) -> Self {
        Self(serde_qs::from_str::<HashMap<String, String>>(query).unwrap_or_default())
    }
}

impl Deref for Query {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
