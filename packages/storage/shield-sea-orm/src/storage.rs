use shield::Storage;

pub const SEA_ORM_STORAGE_ID: &str = "sea-orm";

#[derive(Clone, Debug)]
pub struct SeaOrmStorage {}

impl Storage for SeaOrmStorage {
    fn id(&self) -> String {
        SEA_ORM_STORAGE_ID.to_owned()
    }
}
