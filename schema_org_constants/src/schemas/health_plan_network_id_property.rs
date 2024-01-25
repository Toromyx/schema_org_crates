/// <https://schema.org/healthPlanNetworkId>
pub const HEALTH_PLAN_NETWORK_ID_PROPERTY_IRI_HTTP: &str = "http://schema.org/healthPlanNetworkId";
/// <https://schema.org/healthPlanNetworkId>
pub const HEALTH_PLAN_NETWORK_ID_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/healthPlanNetworkId";
/// <https://schema.org/healthPlanNetworkId>
pub const HEALTH_PLAN_NETWORK_ID_PROPERTY_LABEL: &str = "healthPlanNetworkId";
pub struct HealthPlanNetworkIdPropertyIri;
impl PartialEq<&str> for HealthPlanNetworkIdPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_NETWORK_ID_PROPERTY_IRI_HTTP
			|| *other == HEALTH_PLAN_NETWORK_ID_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanNetworkIdPropertyIri> for &str {
	fn eq(&self, other: &HealthPlanNetworkIdPropertyIri) -> bool {
		*self == HEALTH_PLAN_NETWORK_ID_PROPERTY_IRI_HTTP
			|| *self == HEALTH_PLAN_NETWORK_ID_PROPERTY_IRI_HTTPS
	}
}
pub struct HealthPlanNetworkIdPropertyIriOrLabel;
impl PartialEq<&str> for HealthPlanNetworkIdPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanNetworkIdPropertyIri || *other == HEALTH_PLAN_NETWORK_ID_PROPERTY_LABEL
	}
}
impl PartialEq<HealthPlanNetworkIdPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanNetworkIdPropertyIriOrLabel) -> bool {
		*self == HealthPlanNetworkIdPropertyIri || *self == HEALTH_PLAN_NETWORK_ID_PROPERTY_LABEL
	}
}
