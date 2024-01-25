/// <https://schema.org/healthPlanCoinsuranceRate>
pub const HEALTH_PLAN_COINSURANCE_RATE_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/healthPlanCoinsuranceRate";
/// <https://schema.org/healthPlanCoinsuranceRate>
pub const HEALTH_PLAN_COINSURANCE_RATE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/healthPlanCoinsuranceRate";
/// <https://schema.org/healthPlanCoinsuranceRate>
pub const HEALTH_PLAN_COINSURANCE_RATE_PROPERTY_LABEL: &str = "healthPlanCoinsuranceRate";
pub struct HealthPlanCoinsuranceRatePropertyIri;
impl PartialEq<&str> for HealthPlanCoinsuranceRatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_COINSURANCE_RATE_PROPERTY_IRI_HTTP
			|| *other == HEALTH_PLAN_COINSURANCE_RATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanCoinsuranceRatePropertyIri> for &str {
	fn eq(&self, other: &HealthPlanCoinsuranceRatePropertyIri) -> bool {
		*self == HEALTH_PLAN_COINSURANCE_RATE_PROPERTY_IRI_HTTP
			|| *self == HEALTH_PLAN_COINSURANCE_RATE_PROPERTY_IRI_HTTPS
	}
}
pub struct HealthPlanCoinsuranceRatePropertyIriOrLabel;
impl PartialEq<&str> for HealthPlanCoinsuranceRatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanCoinsuranceRatePropertyIri
			|| *other == HEALTH_PLAN_COINSURANCE_RATE_PROPERTY_LABEL
	}
}
impl PartialEq<HealthPlanCoinsuranceRatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanCoinsuranceRatePropertyIriOrLabel) -> bool {
		*self == HealthPlanCoinsuranceRatePropertyIri
			|| *self == HEALTH_PLAN_COINSURANCE_RATE_PROPERTY_LABEL
	}
}
