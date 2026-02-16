use anyhow::{Error, Result};
use fallible_iterator::FallibleIterator;
use serde::{Deserialize, Serialize};

use crate::content::Content;
use crate::thesis::Thesis;

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum ExternalizeRelationsNodes {
    None,
    Related,
    All,
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum ShowNodesReferences {
    None,
    Mentioned,
    All,
}

#[derive(Serialize, Deserialize)]
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
            let thesis_id_string = thesis.id()?.to_string();
            let node_header_text = if let Some(ref alias) = thesis.alias {
                alias.0.clone()
            } else {
                thesis_id_string.clone()
            };
            match thesis.content {
                Content::Text(ref text) => {
                    let node_body_text = text.composed();
                    let node_header =
                        format!(r#"<TR><TD BORDER="1" SIDES="b">{node_header_text}</TD></TR>"#,);
                    let node_label = format!(
                        r#"<TABLE BORDER="2" CELLSPACING="0" CELLPADDING="8">{}<TR><TD BORDER="0">{}</TD></TR></TABLE>"#,
                        node_header, node_body_text
                    );
                    Some(
                        format!(
                            "\n\t\"{}\" [label=<{}>, shape=plaintext];", // node definition
                            thesis_id_string, node_label
                        ) + &thesis // node references arrows definitions
                            .references()
                            .iter()
                            .map(|referenced_thesis_id| format!("\n\t\"{thesis_id_string}\" -> \"{}\" [arrowhead=none, color=\"grey\" style=dotted];", referenced_thesis_id.to_string()))
                            .collect::<Vec<_>>()
                            .join(""),
                    )
                }
                Content::Relation(ref relation) => {
                    let node_label = format!(
                        r#"<TABLE CELLSPACING="0" STYLE="dashed"><TR><TD SIDES="b" STYLE="dashed">{node_header_text}</TD></TR><TR><TD BORDER="0">{}</TD></TR></TABLE>"#,
                        relation.kind.0
                    );
                    Some(format!(
                        "\n\t\"{thesis_id_string}\" [label=<{node_label}>, shape=plaintext];\n\t\"{}\" -> \"{}\" [dir=back, arrowtail=tee];\n\t\"{}\" -> \"{}\";",
                        relation.from.to_string(), // arrow to relation node
                        thesis_id_string,
                        thesis_id_string, // arrow from relation node
                        relation.to.to_string()
                    ))
                }
            }
        } else {
            None
        })
    }
}
