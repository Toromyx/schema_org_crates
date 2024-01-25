/// <https://schema.org/healthPlanCoinsuranceOption>
pub const HEALTH_PLAN_COINSURANCE_OPTION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/healthPlanCoinsuranceOption";
/// <https://schema.org/healthPlanCoinsuranceOption>
pub const HEALTH_PLAN_COINSURANCE_OPTION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/healthPlanCoinsuranceOption";
/// <https://schema.org/healthPlanCoinsuranceOption>
pub const HEALTH_PLAN_COINSURANCE_OPTION_PROPERTY_LABEL: &str = "healthPlanCoinsuranceOption";
pub struct HealthPlanCoinsuranceOptionPropertyIri;
impl PartialEq<&str> for HealthPlanCoinsuranceOptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_COINSURANCE_OPTION_PROPERTY_IRI_HTTP
			|| *other == HEALTH_PLAN_COINSURANCE_OPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanCoinsuranceOptionPropertyIri> for &str {
	fn eq(&self, other: &HealthPlanCoinsuranceOptionPropertyIri) -> bool {
		*self == HEALTH_PLAN_COINSURANCE_OPTION_PROPERTY_IRI_HTTP
			|| *self == HEALTH_PLAN_COINSURANCE_OPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct HealthPlanCoinsuranceOptionPropertyIriOrLabel;
impl PartialEq<&str> for HealthPlanCoinsuranceOptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanCoinsuranceOptionPropertyIri
			|| *other == HEALTH_PLAN_COINSURANCE_OPTION_PROPERTY_LABEL
	}
}
impl PartialEq<HealthPlanCoinsuranceOptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanCoinsuranceOptionPropertyIriOrLabel) -> bool {
		*self == HealthPlanCoinsuranceOptionPropertyIri
			|| *self == HEALTH_PLAN_COINSURANCE_OPTION_PROPERTY_LABEL
	}
}
