/// <https://schema.org/includesHealthPlanNetwork>
pub const INCLUDES_HEALTH_PLAN_NETWORK_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/includesHealthPlanNetwork";
/// <https://schema.org/includesHealthPlanNetwork>
pub const INCLUDES_HEALTH_PLAN_NETWORK_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/includesHealthPlanNetwork";
/// <https://schema.org/includesHealthPlanNetwork>
pub const INCLUDES_HEALTH_PLAN_NETWORK_PROPERTY_LABEL: &str = "includesHealthPlanNetwork";
pub struct IncludesHealthPlanNetworkPropertyIri;
impl PartialEq<&str> for IncludesHealthPlanNetworkPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INCLUDES_HEALTH_PLAN_NETWORK_PROPERTY_IRI_HTTP
			|| *other == INCLUDES_HEALTH_PLAN_NETWORK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IncludesHealthPlanNetworkPropertyIri> for &str {
	fn eq(&self, other: &IncludesHealthPlanNetworkPropertyIri) -> bool {
		*self == INCLUDES_HEALTH_PLAN_NETWORK_PROPERTY_IRI_HTTP
			|| *self == INCLUDES_HEALTH_PLAN_NETWORK_PROPERTY_IRI_HTTPS
	}
}
pub struct IncludesHealthPlanNetworkPropertyIriOrLabel;
impl PartialEq<&str> for IncludesHealthPlanNetworkPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IncludesHealthPlanNetworkPropertyIri
			|| *other == INCLUDES_HEALTH_PLAN_NETWORK_PROPERTY_LABEL
	}
}
impl PartialEq<IncludesHealthPlanNetworkPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IncludesHealthPlanNetworkPropertyIriOrLabel) -> bool {
		*self == IncludesHealthPlanNetworkPropertyIri
			|| *self == INCLUDES_HEALTH_PLAN_NETWORK_PROPERTY_LABEL
	}
}
