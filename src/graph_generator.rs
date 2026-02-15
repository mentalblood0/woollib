use anyhow::{Error, Result};
use fallible_iterator::FallibleIterator;

use crate::content::Content;
use crate::read_transaction::ReadTransactionMethods;
use crate::relation::Relation;
use crate::text::Text;
use crate::thesis::Thesis;

#[derive(PartialEq, Eq)]
pub enum ExternalizeRelationsNodes {
    None,
    Related,
    All,
}

#[derive(PartialEq, Eq)]
pub enum ShowNodesReferences {
    None,
    Mentioned,
    All,
}

pub struct GraphGeneratorConfig {
    pub wrap_width: u16,
    pub externalize_relations_nodes: ExternalizeRelationsNodes,
    pub show_nodes_references: ShowNodesReferences,
}

pub struct GraphGenerator<'a> {
    pub config: &'a GraphGeneratorConfig,
    pub theses_iterator: &'a mut dyn FallibleIterator<Item = Thesis, Error = Error>,
}

impl<'a> FallibleIterator for GraphGenerator<'a> {
    type Item = String;
    type Error = Error;

    fn next(&mut self) -> Result<Option<Self::Item>> {
        Ok(if let Some(thesis) = self.theses_iterator.next()? {
            let node_id_string = thesis.id()?.to_string();
            let node_header_text = if let Some(ref alias) = thesis.alias {
                alias.0.clone()
            } else {
                node_id_string.clone()
            };
            match thesis.content {
                Content::Text(ref text) => {
                    let node_body_text = text.composed();
                    let node_header = format!(
                        r#"<TR><TD BORDER="1" SIDES="b">{}</TD></TR>"#,
                        node_header_text
                    );
                    let node_label = format!(
                        r#"<TABLE BORDER="2" CELLSPACING="0" CELLPADDING="8">{}<TR><TD BORDER="0">#{}</TD></TR></TABLE>"#,
                        node_header, node_body_text
                    );
                    Some(
                        format!(
                            r#"\n\t"{}" [label=<{}>, shape=plaintext];"#, // node definition
                            node_id_string, node_label
                        ) + &thesis // node references arrows definitions
                            .references()
                            .iter()
                            .map(|referenced_thesis_id| {
                                format!("\n\t{}", referenced_thesis_id.to_string())
                            })
                            .collect::<Vec<_>>()
                            .join(""),
                    )
                }
                Content::Relation(ref relation) => None,
            }
        } else {
            None
        })
    }
}
