/// <https://schema.org/healthPlanCopay>
pub const HEALTH_PLAN_COPAY_PROPERTY_IRI_HTTP: &str = "http://schema.org/healthPlanCopay";
/// <https://schema.org/healthPlanCopay>
pub const HEALTH_PLAN_COPAY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/healthPlanCopay";
/// <https://schema.org/healthPlanCopay>
pub const HEALTH_PLAN_COPAY_PROPERTY_LABEL: &str = "healthPlanCopay";
pub struct HealthPlanCopayPropertyIri;
impl PartialEq<&str> for HealthPlanCopayPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_COPAY_PROPERTY_IRI_HTTP
			|| *other == HEALTH_PLAN_COPAY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanCopayPropertyIri> for &str {
	fn eq(&self, other: &HealthPlanCopayPropertyIri) -> bool {
		*self == HEALTH_PLAN_COPAY_PROPERTY_IRI_HTTP
			|| *self == HEALTH_PLAN_COPAY_PROPERTY_IRI_HTTPS
	}
}
pub struct HealthPlanCopayPropertyIriOrLabel;
impl PartialEq<&str> for HealthPlanCopayPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanCopayPropertyIri || *other == HEALTH_PLAN_COPAY_PROPERTY_LABEL
	}
}
impl PartialEq<HealthPlanCopayPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanCopayPropertyIriOrLabel) -> bool {
		*self == HealthPlanCopayPropertyIri || *self == HEALTH_PLAN_COPAY_PROPERTY_LABEL
	}
}
