/// <https://schema.org/HealthCare>
pub const HEALTH_CARE_IRI_HTTP: &str = "http://schema.org/HealthCare";
/// <https://schema.org/HealthCare>
pub const HEALTH_CARE_IRI_HTTPS: &str = "https://schema.org/HealthCare";
/// <https://schema.org/HealthCare>
pub const HEALTH_CARE_LABEL: &str = "HealthCare";
pub struct HealthCareIri;
impl PartialEq<&str> for HealthCareIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_CARE_IRI_HTTP || *other == HEALTH_CARE_IRI_HTTPS
	}
}
impl PartialEq<HealthCareIri> for &str {
	fn eq(&self, other: &HealthCareIri) -> bool {
		*self == HEALTH_CARE_IRI_HTTP || *self == HEALTH_CARE_IRI_HTTPS
	}
}
pub struct HealthCareIriOrLabel;
impl PartialEq<&str> for HealthCareIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthCareIri || *other == HEALTH_CARE_LABEL
	}
}
impl PartialEq<HealthCareIriOrLabel> for &str {
	fn eq(&self, other: &HealthCareIriOrLabel) -> bool {
		*self == HealthCareIri || *self == HEALTH_CARE_LABEL
	}
}
