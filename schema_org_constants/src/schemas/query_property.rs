/// <https://schema.org/query>
pub const QUERY_PROPERTY_IRI_HTTP: &str = "http://schema.org/query";
/// <https://schema.org/query>
pub const QUERY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/query";
/// <https://schema.org/query>
pub const QUERY_PROPERTY_LABEL: &str = "query";
pub struct QueryPropertyIri;
impl PartialEq<&str> for QueryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == QUERY_PROPERTY_IRI_HTTP || *other == QUERY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<QueryPropertyIri> for &str {
	fn eq(&self, other: &QueryPropertyIri) -> bool {
		*self == QUERY_PROPERTY_IRI_HTTP || *self == QUERY_PROPERTY_IRI_HTTPS
	}
}
pub struct QueryPropertyIriOrLabel;
impl PartialEq<&str> for QueryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == QueryPropertyIri || *other == QUERY_PROPERTY_LABEL
	}
}
impl PartialEq<QueryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &QueryPropertyIriOrLabel) -> bool {
		*self == QueryPropertyIri || *self == QUERY_PROPERTY_LABEL
	}
}
