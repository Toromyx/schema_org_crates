mod as_literal;
mod as_named_node;
mod into_solutions;

use as_literal::AsLiteral;
use as_named_node::AsNamedNode;
use into_solutions::IntoSolutions;
use oxigraph::{
	sparql::{QueryResults, QuerySolution},
	store::Store,
};

/// The prefixes/namespaces used in the schema.org RDF
const PREFIXES: &str = r#"
PREFIX schema: <https://schema.org/>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
"#;

fn iri_from_solution(solution: &QuerySolution) -> String {
	solution
		.get("node")
		.expect("The ?node variable should exist within the query solution.")
		.as_named_node()
		.as_str()
		.to_string()
}

fn label_from_solution(solution: &QuerySolution) -> String {
	solution
		.get("label")
		.expect("The ?label variable should exist within the query solution.")
		.as_literal()
		.value()
		.to_string()
}

fn in_attic_from_solution(solution: &QuerySolution) -> bool {
	solution
		.get("section")
		.is_some_and(|term| term.as_named_node().as_str() == "https://attic.schema.org")
}

#[derive(Debug, Clone)]
pub struct SchemaQuerySolution {
	pub iri: String,
	pub label: String,
	pub in_attic: bool,
}

impl From<&QuerySolution> for SchemaQuerySolution {
	fn from(value: &QuerySolution) -> Self {
		Self {
			iri: iri_from_solution(value),
			label: label_from_solution(value),
			in_attic: in_attic_from_solution(value),
		}
	}
}

pub struct EnumerationVariantSolution {
	pub iri: String,
	pub label: String,
	pub in_attic: bool,
}

impl From<&QuerySolution> for EnumerationVariantSolution {
	fn from(value: &QuerySolution) -> Self {
		Self {
			iri: iri_from_solution(value),
			label: label_from_solution(value),
			in_attic: in_attic_from_solution(value),
		}
	}
}

struct CountSolution(u64);

impl From<QueryResults> for CountSolution {
	fn from(value: QueryResults) -> Self {
		Self(
			value
				.into_solutions()
				.pop()
				.expect("There should always be a query solution within a count query.")
				.get("count")
				.expect("The ?count variable should exist within a count query solution.")
				.as_literal()
				.value()
				.parse::<u64>()
				.expect("The ?count literal should be parsable as an unsigned integer."),
		)
	}
}

pub trait SchemaQueries {
	fn get_schemas(&self) -> Vec<SchemaQuerySolution>;

	fn is_property(&self, iri: &str) -> bool;

	/// Query for schemas superseding the schema.
	fn get_superseded_by(&self, iri: &str) -> Vec<SchemaQuerySolution>;
}

impl SchemaQueries for Store {
	fn get_schemas(&self) -> Vec<SchemaQuerySolution> {
		let query = format!(
			r#"
{}
SELECT
	?node
	?label
	?section
WHERE {{
	?node rdfs:label ?label .
	OPTIONAL {{ ?node schema:isPartOf ?section . }}
}}
"#,
			PREFIXES
		);
		self.query(&query)
			.unwrap()
			.into_solutions()
			.iter()
			.map(SchemaQuerySolution::from)
			.collect()
	}

	fn is_property(&self, iri: &str) -> bool {
		let query = format!(
			r#"
{}
SELECT
	(COUNT(*) AS ?count)
WHERE {{
	<{}> a rdf:Property .
}}
"#,
			PREFIXES, iri
		);
		let count_solution: CountSolution = self.query(&query).unwrap().into();
		count_solution.0 > 0
	}

	fn get_superseded_by(&self, iri: &str) -> Vec<SchemaQuerySolution> {
		let query = format!(
			r#"
{}
SELECT
	?node
	?label
	?section
WHERE {{
	<{}> schema:supersededBy ?node .
	?node rdfs:label ?label .
	OPTIONAL {{ ?node schema:isPartOf ?section . }}
}}
"#,
			PREFIXES, iri
		);
		self.query(&query)
			.unwrap()
			.into_solutions()
			.iter()
			.map(SchemaQuerySolution::from)
			.collect()
	}
}
