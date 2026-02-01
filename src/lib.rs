mod id_serializer;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use trove::{Chest, ChestConfig};

pub struct Sweater {
    pub chest: Chest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SweaterConfig {
    pub chest: ChestConfig,
}

impl Sweater {
    pub fn new(config: SweaterConfig) -> Result<Self> {
        Ok(Self {
            chest: Chest::new(config.chest.clone()).with_context(|| {
                format!(
                    "Can not create sweater with chest config {:?}",
                    config.chest
                )
            })?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Id(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mention {
    mentioned: Id,
    inside: Id,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Text(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelationKind(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relation {
    from: Id,
    to: Id,
    kind: RelationKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Content {
    Text(Text),
    Relation(Relation),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thesis {
    pub content: Content,
    pub tags: Vec<Tag>,
}
