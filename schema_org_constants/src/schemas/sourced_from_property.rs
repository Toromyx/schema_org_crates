/// <https://schema.org/sourcedFrom>
pub const SOURCED_FROM_PROPERTY_IRI_HTTP: &str = "http://schema.org/sourcedFrom";
/// <https://schema.org/sourcedFrom>
pub const SOURCED_FROM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sourcedFrom";
/// <https://schema.org/sourcedFrom>
pub const SOURCED_FROM_PROPERTY_LABEL: &str = "sourcedFrom";
pub struct SourcedFromPropertyIri;
impl PartialEq<&str> for SourcedFromPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SOURCED_FROM_PROPERTY_IRI_HTTP || *other == SOURCED_FROM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SourcedFromPropertyIri> for &str {
	fn eq(&self, other: &SourcedFromPropertyIri) -> bool {
		*self == SOURCED_FROM_PROPERTY_IRI_HTTP || *self == SOURCED_FROM_PROPERTY_IRI_HTTPS
	}
}
pub struct SourcedFromPropertyIriOrLabel;
impl PartialEq<&str> for SourcedFromPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SourcedFromPropertyIri || *other == SOURCED_FROM_PROPERTY_LABEL
	}
}
impl PartialEq<SourcedFromPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SourcedFromPropertyIriOrLabel) -> bool {
		*self == SourcedFromPropertyIri || *self == SOURCED_FROM_PROPERTY_LABEL
	}
}
