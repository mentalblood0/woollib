use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use trove::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone, bincode::Encode)]
pub struct Mention {
    pub mentioned: ObjectId,
    pub inside: ObjectId,
}

impl Mention {
    pub fn id(&self) -> Result<ObjectId> {
        Ok(ObjectId {
            value: xxhash_rust::xxh3::xxh3_128(
                &bincode::encode_to_vec(self, bincode::config::standard())
                    .with_context(|| format!("Can not binary encode Mention {self:?} in order to compute it's ObjectId as it's binary representation hash"))?,
            )
            .to_be_bytes(),
        })
    }
}
