/// <https://schema.org/smokingAllowed>
pub const SMOKING_ALLOWED_PROPERTY_IRI_HTTP: &str = "http://schema.org/smokingAllowed";
/// <https://schema.org/smokingAllowed>
pub const SMOKING_ALLOWED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/smokingAllowed";
/// <https://schema.org/smokingAllowed>
pub const SMOKING_ALLOWED_PROPERTY_LABEL: &str = "smokingAllowed";
pub struct SmokingAllowedPropertyIri;
impl PartialEq<&str> for SmokingAllowedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SMOKING_ALLOWED_PROPERTY_IRI_HTTP || *other == SMOKING_ALLOWED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SmokingAllowedPropertyIri> for &str {
	fn eq(&self, other: &SmokingAllowedPropertyIri) -> bool {
		*self == SMOKING_ALLOWED_PROPERTY_IRI_HTTP || *self == SMOKING_ALLOWED_PROPERTY_IRI_HTTPS
	}
}
pub struct SmokingAllowedPropertyIriOrLabel;
impl PartialEq<&str> for SmokingAllowedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SmokingAllowedPropertyIri || *other == SMOKING_ALLOWED_PROPERTY_LABEL
	}
}
impl PartialEq<SmokingAllowedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SmokingAllowedPropertyIriOrLabel) -> bool {
		*self == SmokingAllowedPropertyIri || *self == SMOKING_ALLOWED_PROPERTY_LABEL
	}
}
