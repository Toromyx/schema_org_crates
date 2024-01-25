/// <https://schema.org/performers>
#[deprecated = "This schema is superseded by <https://schema.org/performer>."]
pub const PERFORMERS_PROPERTY_IRI_HTTP: &str = "http://schema.org/performers";
/// <https://schema.org/performers>
#[deprecated = "This schema is superseded by <https://schema.org/performer>."]
pub const PERFORMERS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/performers";
/// <https://schema.org/performers>
#[deprecated = "This schema is superseded by <https://schema.org/performer>."]
pub const PERFORMERS_PROPERTY_LABEL: &str = "performers";
pub struct PerformersPropertyIri;
impl PartialEq<&str> for PerformersPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERFORMERS_PROPERTY_IRI_HTTP || *other == PERFORMERS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PerformersPropertyIri> for &str {
	fn eq(&self, other: &PerformersPropertyIri) -> bool {
		*self == PERFORMERS_PROPERTY_IRI_HTTP || *self == PERFORMERS_PROPERTY_IRI_HTTPS
	}
}
pub struct PerformersPropertyIriOrLabel;
impl PartialEq<&str> for PerformersPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PerformersPropertyIri || *other == PERFORMERS_PROPERTY_LABEL
	}
}
impl PartialEq<PerformersPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PerformersPropertyIriOrLabel) -> bool {
		*self == PerformersPropertyIri || *self == PERFORMERS_PROPERTY_LABEL
	}
}
