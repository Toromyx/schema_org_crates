/// <https://schema.org/underName>
pub const UNDER_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/underName";
/// <https://schema.org/underName>
pub const UNDER_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/underName";
/// <https://schema.org/underName>
pub const UNDER_NAME_PROPERTY_LABEL: &str = "underName";
pub struct UnderNamePropertyIri;
impl PartialEq<&str> for UnderNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UNDER_NAME_PROPERTY_IRI_HTTP || *other == UNDER_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UnderNamePropertyIri> for &str {
	fn eq(&self, other: &UnderNamePropertyIri) -> bool {
		*self == UNDER_NAME_PROPERTY_IRI_HTTP || *self == UNDER_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct UnderNamePropertyIriOrLabel;
impl PartialEq<&str> for UnderNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UnderNamePropertyIri || *other == UNDER_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<UnderNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &UnderNamePropertyIriOrLabel) -> bool {
		*self == UnderNamePropertyIri || *self == UNDER_NAME_PROPERTY_LABEL
	}
}
