/// <https://schema.org/additionalProperty>
pub const ADDITIONAL_PROPERTY_PROPERTY_IRI_HTTP: &str = "http://schema.org/additionalProperty";
/// <https://schema.org/additionalProperty>
pub const ADDITIONAL_PROPERTY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/additionalProperty";
/// <https://schema.org/additionalProperty>
pub const ADDITIONAL_PROPERTY_PROPERTY_LABEL: &str = "additionalProperty";
pub struct AdditionalPropertyPropertyIri;
impl PartialEq<&str> for AdditionalPropertyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADDITIONAL_PROPERTY_PROPERTY_IRI_HTTP
			|| *other == ADDITIONAL_PROPERTY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AdditionalPropertyPropertyIri> for &str {
	fn eq(&self, other: &AdditionalPropertyPropertyIri) -> bool {
		*self == ADDITIONAL_PROPERTY_PROPERTY_IRI_HTTP
			|| *self == ADDITIONAL_PROPERTY_PROPERTY_IRI_HTTPS
	}
}
pub struct AdditionalPropertyPropertyIriOrLabel;
impl PartialEq<&str> for AdditionalPropertyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AdditionalPropertyPropertyIri || *other == ADDITIONAL_PROPERTY_PROPERTY_LABEL
	}
}
impl PartialEq<AdditionalPropertyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AdditionalPropertyPropertyIriOrLabel) -> bool {
		*self == AdditionalPropertyPropertyIri || *self == ADDITIONAL_PROPERTY_PROPERTY_LABEL
	}
}
