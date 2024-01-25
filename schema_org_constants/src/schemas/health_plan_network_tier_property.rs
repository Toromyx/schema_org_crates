/// <https://schema.org/healthPlanNetworkTier>
pub const HEALTH_PLAN_NETWORK_TIER_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/healthPlanNetworkTier";
/// <https://schema.org/healthPlanNetworkTier>
pub const HEALTH_PLAN_NETWORK_TIER_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/healthPlanNetworkTier";
/// <https://schema.org/healthPlanNetworkTier>
pub const HEALTH_PLAN_NETWORK_TIER_PROPERTY_LABEL: &str = "healthPlanNetworkTier";
pub struct HealthPlanNetworkTierPropertyIri;
impl PartialEq<&str> for HealthPlanNetworkTierPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_NETWORK_TIER_PROPERTY_IRI_HTTP
			|| *other == HEALTH_PLAN_NETWORK_TIER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanNetworkTierPropertyIri> for &str {
	fn eq(&self, other: &HealthPlanNetworkTierPropertyIri) -> bool {
		*self == HEALTH_PLAN_NETWORK_TIER_PROPERTY_IRI_HTTP
			|| *self == HEALTH_PLAN_NETWORK_TIER_PROPERTY_IRI_HTTPS
	}
}
pub struct HealthPlanNetworkTierPropertyIriOrLabel;
impl PartialEq<&str> for HealthPlanNetworkTierPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanNetworkTierPropertyIri
			|| *other == HEALTH_PLAN_NETWORK_TIER_PROPERTY_LABEL
	}
}
impl PartialEq<HealthPlanNetworkTierPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanNetworkTierPropertyIriOrLabel) -> bool {
		*self == HealthPlanNetworkTierPropertyIri
			|| *self == HEALTH_PLAN_NETWORK_TIER_PROPERTY_LABEL
	}
}
