use anyhow::Result;
use serde::{Deserialize, Serialize};
use trove::ObjectId;

use crate::alias::Alias;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reference {
    Alias(Alias),
    ObjectId(ObjectId),
}

impl Reference {
    pub fn new(input: &str) -> Result<Self> {
        if let Ok(alias) = Alias(input.to_string()).validated() {
            Ok(Self::Alias(alias.to_owned()))
        } else {
            Ok(Self::ObjectId(serde_json::from_str(&format!(
                "\"{}\"",
                input
            ))?))
        }
    }

    pub fn validated(&self) -> Result<&Self> {
        match self {
            Reference::Alias(alias) => {
                alias.validated()?;
            }
            Reference::ObjectId(_) => {}
        }
        Ok(self)
    }
}
