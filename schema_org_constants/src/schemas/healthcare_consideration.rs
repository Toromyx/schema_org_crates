/// <https://schema.org/HealthcareConsideration>
pub const HEALTHCARE_CONSIDERATION_IRI_HTTP: &str = "http://schema.org/HealthcareConsideration";
/// <https://schema.org/HealthcareConsideration>
pub const HEALTHCARE_CONSIDERATION_IRI_HTTPS: &str = "https://schema.org/HealthcareConsideration";
/// <https://schema.org/HealthcareConsideration>
pub const HEALTHCARE_CONSIDERATION_LABEL: &str = "HealthcareConsideration";
pub struct HealthcareConsiderationIri;
impl PartialEq<&str> for HealthcareConsiderationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTHCARE_CONSIDERATION_IRI_HTTP || *other == HEALTHCARE_CONSIDERATION_IRI_HTTPS
	}
}
impl PartialEq<HealthcareConsiderationIri> for &str {
	fn eq(&self, other: &HealthcareConsiderationIri) -> bool {
		*self == HEALTHCARE_CONSIDERATION_IRI_HTTP || *self == HEALTHCARE_CONSIDERATION_IRI_HTTPS
	}
}
pub struct HealthcareConsiderationIriOrLabel;
impl PartialEq<&str> for HealthcareConsiderationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthcareConsiderationIri || *other == HEALTHCARE_CONSIDERATION_LABEL
	}
}
impl PartialEq<HealthcareConsiderationIriOrLabel> for &str {
	fn eq(&self, other: &HealthcareConsiderationIriOrLabel) -> bool {
		*self == HealthcareConsiderationIri || *self == HEALTHCARE_CONSIDERATION_LABEL
	}
}
