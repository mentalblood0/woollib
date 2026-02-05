use anyhow::Result;
use trove::ObjectId;

use super::sweater::SweaterConfig;
use super::thesis::Thesis;

pub struct ReadTransaction<'a> {
    pub chest_transaction: &'a trove::ReadTransaction<'a>,
    pub sweater_config: SweaterConfig,
}

#[macro_export]
macro_rules! define_read_methods {
    () => {
        pub fn get_thesis(&self, thesis_id: &ObjectId) -> Result<Option<Thesis>> {
            if let Some(thesis_json_value) = self.chest_transaction.get(thesis_id, &vec![])? {
                Ok(Some(serde_json::from_value(thesis_json_value).unwrap()))
            } else {
                Ok(None)
            }
        }
    };
}

impl ReadTransaction<'_> {
    define_read_methods!();
}
