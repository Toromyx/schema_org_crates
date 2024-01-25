/// <https://schema.org/additionalType>
pub const ADDITIONAL_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/additionalType";
/// <https://schema.org/additionalType>
pub const ADDITIONAL_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/additionalType";
/// <https://schema.org/additionalType>
pub const ADDITIONAL_TYPE_PROPERTY_LABEL: &str = "additionalType";
pub struct AdditionalTypePropertyIri;
impl PartialEq<&str> for AdditionalTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADDITIONAL_TYPE_PROPERTY_IRI_HTTP || *other == ADDITIONAL_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AdditionalTypePropertyIri> for &str {
	fn eq(&self, other: &AdditionalTypePropertyIri) -> bool {
		*self == ADDITIONAL_TYPE_PROPERTY_IRI_HTTP || *self == ADDITIONAL_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct AdditionalTypePropertyIriOrLabel;
impl PartialEq<&str> for AdditionalTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AdditionalTypePropertyIri || *other == ADDITIONAL_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<AdditionalTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AdditionalTypePropertyIriOrLabel) -> bool {
		*self == AdditionalTypePropertyIri || *self == ADDITIONAL_TYPE_PROPERTY_LABEL
	}
}
