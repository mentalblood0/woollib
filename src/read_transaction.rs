use anyhow::Result;
use trove::{IndexRecordType, ObjectId, path_segments};

use crate::alias::Alias;
use crate::sweater::SweaterConfig;
use crate::thesis::Thesis;

pub struct ReadTransaction<'a> {
    pub chest_transaction: &'a trove::ReadTransaction<'a>,
    pub sweater_config: SweaterConfig,
}

#[macro_export]
macro_rules! define_read_methods {
    () => {
        fn get_thesis(&self, thesis_id: &ObjectId) -> Result<Option<Thesis>> {
            if let Some(thesis_json_value) = self.chest_transaction.get(thesis_id, &vec![])? {
                Ok(Some(serde_json::from_value(thesis_json_value).unwrap()))
            } else {
                Ok(None)
            }
        }

        fn get_thesis_id_by_alias(&self, alias: &Alias) -> Result<Option<ObjectId>> {
            Ok(self
                .chest_transaction
                .select(
                    &vec![(
                        IndexRecordType::Direct,
                        path_segments!("alias"),
                        serde_json::to_value(alias)?,
                    )],
                    &vec![],
                    None,
                )?
                .next()?)
        }
    };
}

pub trait ReadTransactionMethods {
    fn get_thesis(&self, thesis_id: &ObjectId) -> Result<Option<Thesis>>;
    fn get_thesis_id_by_alias(&self, alias: &Alias) -> Result<Option<ObjectId>>;
}

impl ReadTransactionMethods for ReadTransaction<'_> {
    define_read_methods!();
}
