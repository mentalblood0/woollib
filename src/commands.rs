use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result, anyhow};
use regex::Regex;
use serde::{Deserialize, Serialize};
use trove::ObjectId;

use super::relation::RelationKind;
use super::tag::Tag;
use super::text::Text;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Alias(String);

static ALIAS_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();

impl Alias {
    pub fn validate(&self) -> Result<()> {
        let sentence_regex = ALIAS_REGEX.get_or_init(|| {
            Regex::new(r#"^\S+$"#)
                .with_context(|| "Can not compile regular expression for thesis alias validation")
                .unwrap()
        });
        if sentence_regex.is_match(&self.0) {
            Ok(())
        } else {
            Err(anyhow!(
                "Alias must be sequence of one or more non-whitespace characters, so {:?} does not seem to be text",
                self.0
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThesisReference {
    Alias(Alias),
    ObjectId(ObjectId),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AddTextThesis {
    pub alias: Option<Alias>,
    pub text: Text,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AddRelationThesis {
    pub alias: Option<Alias>,
    pub from: ThesisReference,
    pub to: ThesisReference,
    pub kind: RelationKind,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RemoveThesis {
    pub thesis_id: ObjectId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AddTag {
    pub thesis_reference: ThesisReference,
    pub tag: Tag,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RemoveTag {
    pub thesis_id: ObjectId,
    pub tag: Tag,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Command {
    AddTextThesis(AddTextThesis),
    AddRelationThesis(AddRelationThesis),
    AddTag(AddTag),
    RemoveThesis(RemoveThesis),
    RemoveTag(RemoveThesis),
}

static COMMANDS_SPLIT_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
static COMMAND_FIRST_LINE_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();

pub fn parse_commands_batch(
    input: &str,
    supported_relations_kinds: &BTreeSet<RelationKind>,
) -> Result<Vec<Command>> {
    let commands_split_regex = COMMANDS_SPLIT_REGEX.get_or_init(|| {
        Regex::new(r#"(\r?\n|\r){2,}"#)
            .with_context(|| "Can not compile regular expression for commands splitting")
            .unwrap()
    });
    let command_first_line_regex = COMMAND_FIRST_LINE_REGEX.get_or_init(|| {
        Regex::new(r#"^ *(\+|-|#|\^) +([^ ]+) *$"#)
            .with_context(|| "Can not compile regular expression for commands splitting")
            .unwrap()
    });
    let mut result = Vec::new();
    let mut alias_to_index_in_result: BTreeMap<Alias, usize> = BTreeMap::new();
    for (paragraph_index, paragraph) in commands_split_regex
        .split(input)
        .map(|paragraph| paragraph.trim())
        .filter(|paragraph| !paragraph.is_empty())
        .enumerate()
    {
        let lines = paragraph.split('\n').collect::<Vec<_>>();
        if let Some(captures) = command_first_line_regex.captures(lines[0]) {
            let operation_char = captures[1].chars().next().unwrap();
            let alias_option = captures
                .get(1)
                .map(|alias_match| Alias(alias_match.as_str().to_string()));
            if let Some(alias) = alias_option {
                alias.validate().with_context(|| {
                    format!(
                        "Can not parse first line {:?} in {}-nth paragraph {:?}",
                        lines[0],
                        paragraph_index + 1,
                        paragraph
                    )
                })?;
            }
            let command = match (operation_char, lines.len()) {
                ('+', 2) => Ok(Command::AddTextThesis(AddTextThesis {
                    alias: alias_option,
                    text: Text(lines[1].to_string()),
                })),
                ('+', 4) => Ok(Command::AddRelationThesis(AddRelationThesis {
                    alias: alias_option,
                    from: (),
                    to: (),
                    kind: (),
                })),
                _ => Err(anyhow!(
                    "Unsupported operation character and lines count combination ({:?}, {}) in first line {:?} of {}-nth paragraph {:?}, supported combinations are ('+', 2) for adding text thesis, ('+', 4) for adding relation thesis, ('-', 2) for removing thesis, ('#', 3) for adding tag, ('^', 3) for removing tag",
                    operation_char,
                    lines.len(),
                    lines[0],
                    paragraph_index + 1,
                    paragraph
                )),
            }?;
        }
    }
    Ok(result)
}
