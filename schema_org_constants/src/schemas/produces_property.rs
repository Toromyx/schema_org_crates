/// <https://schema.org/produces>
#[deprecated = "This schema is superseded by <https://schema.org/serviceOutput>."]
pub const PRODUCES_PROPERTY_IRI_HTTP: &str = "http://schema.org/produces";
/// <https://schema.org/produces>
#[deprecated = "This schema is superseded by <https://schema.org/serviceOutput>."]
pub const PRODUCES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/produces";
/// <https://schema.org/produces>
#[deprecated = "This schema is superseded by <https://schema.org/serviceOutput>."]
pub const PRODUCES_PROPERTY_LABEL: &str = "produces";
pub struct ProducesPropertyIri;
impl PartialEq<&str> for ProducesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCES_PROPERTY_IRI_HTTP || *other == PRODUCES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProducesPropertyIri> for &str {
	fn eq(&self, other: &ProducesPropertyIri) -> bool {
		*self == PRODUCES_PROPERTY_IRI_HTTP || *self == PRODUCES_PROPERTY_IRI_HTTPS
	}
}
pub struct ProducesPropertyIriOrLabel;
impl PartialEq<&str> for ProducesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProducesPropertyIri || *other == PRODUCES_PROPERTY_LABEL
	}
}
impl PartialEq<ProducesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProducesPropertyIriOrLabel) -> bool {
		*self == ProducesPropertyIri || *self == PRODUCES_PROPERTY_LABEL
	}
}
