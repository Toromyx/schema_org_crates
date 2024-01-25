/// <https://schema.org/additionalName>
pub const ADDITIONAL_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/additionalName";
/// <https://schema.org/additionalName>
pub const ADDITIONAL_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/additionalName";
/// <https://schema.org/additionalName>
pub const ADDITIONAL_NAME_PROPERTY_LABEL: &str = "additionalName";
pub struct AdditionalNamePropertyIri;
impl PartialEq<&str> for AdditionalNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADDITIONAL_NAME_PROPERTY_IRI_HTTP || *other == ADDITIONAL_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AdditionalNamePropertyIri> for &str {
	fn eq(&self, other: &AdditionalNamePropertyIri) -> bool {
		*self == ADDITIONAL_NAME_PROPERTY_IRI_HTTP || *self == ADDITIONAL_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct AdditionalNamePropertyIriOrLabel;
impl PartialEq<&str> for AdditionalNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AdditionalNamePropertyIri || *other == ADDITIONAL_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<AdditionalNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AdditionalNamePropertyIriOrLabel) -> bool {
		*self == AdditionalNamePropertyIri || *self == ADDITIONAL_NAME_PROPERTY_LABEL
	}
}
