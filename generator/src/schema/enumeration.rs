pub mod enumeration_variant;

use std::str::FromStr;

use convert_case::{Case, Casing};
use derivative::Derivative;
use enumeration_variant::EnumerationVariant;
use oxigraph::store::Store;
use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};
use rayon::prelude::*;

use crate::{
    doc_lines::DocLines,
    read::map_schema_name,
    schema::Schema,
    schema_section::SchemaSection,
    serde_attributes::serde_derive,
    sparql::{SchemaQueries, SectionedSchemaQuerySolution},
};

/// A schema.org enumeration.
///
/// This struct contains all information to be quoted into a rust enum.
#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Enumeration {
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub iri: String,
    pub name: String,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub variants: Vec<EnumerationVariant>,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub section: SchemaSection,
}

impl Schema for Enumeration {
    fn module_name() -> &'static str {
        "enumerations"
    }

    fn feature_name(&self) -> String {
        format!("{}-schema", self.name.to_case(Case::Kebab))
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn iri(&self) -> &String {
        &self.iri
    }

    fn section(&self) -> &SchemaSection {
        &self.section
    }

    fn child_feature_names(&self) -> Vec<String> {
        vec![]
    }

    fn read_solutions(store: &Store) -> Vec<SectionedSchemaQuerySolution> {
        store.enumerations_query()
    }

    fn from_solution(store: &Store, solution: SectionedSchemaQuerySolution) -> Self {
        let mut variants: Vec<EnumerationVariant> = store
            .variants_of_enumeration_query(&solution.schema.identifiable.iri)
            .into_par_iter()
            .map(|solution| EnumerationVariant {
                iri: solution.identifiable.iri,
                name: map_schema_name(solution.labeled.label),
            })
            .collect();
        variants.sort_unstable();
        Self {
            iri: solution.schema.identifiable.iri,
            name: map_schema_name(solution.schema.labeled.label),
            variants,
            section: solution.sectioned.section,
        }
    }
}

impl ToTokens for Enumeration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let doc_lines = self.doc_lines_token_stream();
        let serde_derive = serde_derive();
        let name = TokenStream::from_str(&self.name.to_case(Case::UpperCamel)).unwrap();
        let variants = &self.variants;
        tokens.append_all(quote!(
            #doc_lines
            #[cfg_attr(feature = "derive-debug", derive(Debug))]
            #[cfg_attr(feature = "derive-clone", derive(Clone))]
            #serde_derive
            pub enum #name {
                #(#variants),*
            }
        ));
    }
}
