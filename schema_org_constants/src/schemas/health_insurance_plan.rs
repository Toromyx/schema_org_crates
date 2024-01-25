/// <https://schema.org/HealthInsurancePlan>
pub const HEALTH_INSURANCE_PLAN_IRI_HTTP: &str = "http://schema.org/HealthInsurancePlan";
/// <https://schema.org/HealthInsurancePlan>
pub const HEALTH_INSURANCE_PLAN_IRI_HTTPS: &str = "https://schema.org/HealthInsurancePlan";
/// <https://schema.org/HealthInsurancePlan>
pub const HEALTH_INSURANCE_PLAN_LABEL: &str = "HealthInsurancePlan";
pub struct HealthInsurancePlanIri;
impl PartialEq<&str> for HealthInsurancePlanIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_INSURANCE_PLAN_IRI_HTTP || *other == HEALTH_INSURANCE_PLAN_IRI_HTTPS
	}
}
impl PartialEq<HealthInsurancePlanIri> for &str {
	fn eq(&self, other: &HealthInsurancePlanIri) -> bool {
		*self == HEALTH_INSURANCE_PLAN_IRI_HTTP || *self == HEALTH_INSURANCE_PLAN_IRI_HTTPS
	}
}
pub struct HealthInsurancePlanIriOrLabel;
impl PartialEq<&str> for HealthInsurancePlanIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthInsurancePlanIri || *other == HEALTH_INSURANCE_PLAN_LABEL
	}
}
impl PartialEq<HealthInsurancePlanIriOrLabel> for &str {
	fn eq(&self, other: &HealthInsurancePlanIriOrLabel) -> bool {
		*self == HealthInsurancePlanIri || *self == HEALTH_INSURANCE_PLAN_LABEL
	}
}
