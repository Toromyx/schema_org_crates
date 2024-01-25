/// <https://schema.org/greater>
pub const GREATER_PROPERTY_IRI_HTTP: &str = "http://schema.org/greater";
/// <https://schema.org/greater>
pub const GREATER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/greater";
/// <https://schema.org/greater>
pub const GREATER_PROPERTY_LABEL: &str = "greater";
pub struct GreaterPropertyIri;
impl PartialEq<&str> for GreaterPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GREATER_PROPERTY_IRI_HTTP || *other == GREATER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GreaterPropertyIri> for &str {
	fn eq(&self, other: &GreaterPropertyIri) -> bool {
		*self == GREATER_PROPERTY_IRI_HTTP || *self == GREATER_PROPERTY_IRI_HTTPS
	}
}
pub struct GreaterPropertyIriOrLabel;
impl PartialEq<&str> for GreaterPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GreaterPropertyIri || *other == GREATER_PROPERTY_LABEL
	}
}
impl PartialEq<GreaterPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GreaterPropertyIriOrLabel) -> bool {
		*self == GreaterPropertyIri || *self == GREATER_PROPERTY_LABEL
	}
}
