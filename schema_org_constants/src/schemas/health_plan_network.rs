/// <https://schema.org/HealthPlanNetwork>
pub const HEALTH_PLAN_NETWORK_IRI_HTTP: &str = "http://schema.org/HealthPlanNetwork";
/// <https://schema.org/HealthPlanNetwork>
pub const HEALTH_PLAN_NETWORK_IRI_HTTPS: &str = "https://schema.org/HealthPlanNetwork";
/// <https://schema.org/HealthPlanNetwork>
pub const HEALTH_PLAN_NETWORK_LABEL: &str = "HealthPlanNetwork";
pub struct HealthPlanNetworkIri;
impl PartialEq<&str> for HealthPlanNetworkIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_NETWORK_IRI_HTTP || *other == HEALTH_PLAN_NETWORK_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanNetworkIri> for &str {
	fn eq(&self, other: &HealthPlanNetworkIri) -> bool {
		*self == HEALTH_PLAN_NETWORK_IRI_HTTP || *self == HEALTH_PLAN_NETWORK_IRI_HTTPS
	}
}
pub struct HealthPlanNetworkIriOrLabel;
impl PartialEq<&str> for HealthPlanNetworkIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanNetworkIri || *other == HEALTH_PLAN_NETWORK_LABEL
	}
}
impl PartialEq<HealthPlanNetworkIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanNetworkIriOrLabel) -> bool {
		*self == HealthPlanNetworkIri || *self == HEALTH_PLAN_NETWORK_LABEL
	}
}
