/// <https://schema.org/healthPlanCopayOption>
pub const HEALTH_PLAN_COPAY_OPTION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/healthPlanCopayOption";
/// <https://schema.org/healthPlanCopayOption>
pub const HEALTH_PLAN_COPAY_OPTION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/healthPlanCopayOption";
/// <https://schema.org/healthPlanCopayOption>
pub const HEALTH_PLAN_COPAY_OPTION_PROPERTY_LABEL: &str = "healthPlanCopayOption";
pub struct HealthPlanCopayOptionPropertyIri;
impl PartialEq<&str> for HealthPlanCopayOptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_COPAY_OPTION_PROPERTY_IRI_HTTP
			|| *other == HEALTH_PLAN_COPAY_OPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanCopayOptionPropertyIri> for &str {
	fn eq(&self, other: &HealthPlanCopayOptionPropertyIri) -> bool {
		*self == HEALTH_PLAN_COPAY_OPTION_PROPERTY_IRI_HTTP
			|| *self == HEALTH_PLAN_COPAY_OPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct HealthPlanCopayOptionPropertyIriOrLabel;
impl PartialEq<&str> for HealthPlanCopayOptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanCopayOptionPropertyIri
			|| *other == HEALTH_PLAN_COPAY_OPTION_PROPERTY_LABEL
	}
}
impl PartialEq<HealthPlanCopayOptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanCopayOptionPropertyIriOrLabel) -> bool {
		*self == HealthPlanCopayOptionPropertyIri
			|| *self == HEALTH_PLAN_COPAY_OPTION_PROPERTY_LABEL
	}
}
