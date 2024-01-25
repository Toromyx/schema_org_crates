/// <https://schema.org/safetyConsideration>
pub const SAFETY_CONSIDERATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/safetyConsideration";
/// <https://schema.org/safetyConsideration>
pub const SAFETY_CONSIDERATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/safetyConsideration";
/// <https://schema.org/safetyConsideration>
pub const SAFETY_CONSIDERATION_PROPERTY_LABEL: &str = "safetyConsideration";
pub struct SafetyConsiderationPropertyIri;
impl PartialEq<&str> for SafetyConsiderationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SAFETY_CONSIDERATION_PROPERTY_IRI_HTTP
			|| *other == SAFETY_CONSIDERATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SafetyConsiderationPropertyIri> for &str {
	fn eq(&self, other: &SafetyConsiderationPropertyIri) -> bool {
		*self == SAFETY_CONSIDERATION_PROPERTY_IRI_HTTP
			|| *self == SAFETY_CONSIDERATION_PROPERTY_IRI_HTTPS
	}
}
pub struct SafetyConsiderationPropertyIriOrLabel;
impl PartialEq<&str> for SafetyConsiderationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SafetyConsiderationPropertyIri || *other == SAFETY_CONSIDERATION_PROPERTY_LABEL
	}
}
impl PartialEq<SafetyConsiderationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SafetyConsiderationPropertyIriOrLabel) -> bool {
		*self == SafetyConsiderationPropertyIri || *self == SAFETY_CONSIDERATION_PROPERTY_LABEL
	}
}
