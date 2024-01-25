/// <https://schema.org/parents>
#[deprecated = "This schema is superseded by <https://schema.org/parent>."]
pub const PARENTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/parents";
/// <https://schema.org/parents>
#[deprecated = "This schema is superseded by <https://schema.org/parent>."]
pub const PARENTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/parents";
/// <https://schema.org/parents>
#[deprecated = "This schema is superseded by <https://schema.org/parent>."]
pub const PARENTS_PROPERTY_LABEL: &str = "parents";
pub struct ParentsPropertyIri;
impl PartialEq<&str> for ParentsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARENTS_PROPERTY_IRI_HTTP || *other == PARENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ParentsPropertyIri> for &str {
	fn eq(&self, other: &ParentsPropertyIri) -> bool {
		*self == PARENTS_PROPERTY_IRI_HTTP || *self == PARENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct ParentsPropertyIriOrLabel;
impl PartialEq<&str> for ParentsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ParentsPropertyIri || *other == PARENTS_PROPERTY_LABEL
	}
}
impl PartialEq<ParentsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ParentsPropertyIriOrLabel) -> bool {
		*self == ParentsPropertyIri || *self == PARENTS_PROPERTY_LABEL
	}
}
