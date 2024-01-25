/// <https://schema.org/founders>
#[deprecated = "This schema is superseded by <https://schema.org/founder>."]
pub const FOUNDERS_PROPERTY_IRI_HTTP: &str = "http://schema.org/founders";
/// <https://schema.org/founders>
#[deprecated = "This schema is superseded by <https://schema.org/founder>."]
pub const FOUNDERS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/founders";
/// <https://schema.org/founders>
#[deprecated = "This schema is superseded by <https://schema.org/founder>."]
pub const FOUNDERS_PROPERTY_LABEL: &str = "founders";
pub struct FoundersPropertyIri;
impl PartialEq<&str> for FoundersPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOUNDERS_PROPERTY_IRI_HTTP || *other == FOUNDERS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FoundersPropertyIri> for &str {
	fn eq(&self, other: &FoundersPropertyIri) -> bool {
		*self == FOUNDERS_PROPERTY_IRI_HTTP || *self == FOUNDERS_PROPERTY_IRI_HTTPS
	}
}
pub struct FoundersPropertyIriOrLabel;
impl PartialEq<&str> for FoundersPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FoundersPropertyIri || *other == FOUNDERS_PROPERTY_LABEL
	}
}
impl PartialEq<FoundersPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FoundersPropertyIriOrLabel) -> bool {
		*self == FoundersPropertyIri || *self == FOUNDERS_PROPERTY_LABEL
	}
}
