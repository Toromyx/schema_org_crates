/// <https://schema.org/healthPlanCostSharing>
pub const HEALTH_PLAN_COST_SHARING_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/healthPlanCostSharing";
/// <https://schema.org/healthPlanCostSharing>
pub const HEALTH_PLAN_COST_SHARING_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/healthPlanCostSharing";
/// <https://schema.org/healthPlanCostSharing>
pub const HEALTH_PLAN_COST_SHARING_PROPERTY_LABEL: &str = "healthPlanCostSharing";
pub struct HealthPlanCostSharingPropertyIri;
impl PartialEq<&str> for HealthPlanCostSharingPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_COST_SHARING_PROPERTY_IRI_HTTP
			|| *other == HEALTH_PLAN_COST_SHARING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanCostSharingPropertyIri> for &str {
	fn eq(&self, other: &HealthPlanCostSharingPropertyIri) -> bool {
		*self == HEALTH_PLAN_COST_SHARING_PROPERTY_IRI_HTTP
			|| *self == HEALTH_PLAN_COST_SHARING_PROPERTY_IRI_HTTPS
	}
}
pub struct HealthPlanCostSharingPropertyIriOrLabel;
impl PartialEq<&str> for HealthPlanCostSharingPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanCostSharingPropertyIri
			|| *other == HEALTH_PLAN_COST_SHARING_PROPERTY_LABEL
	}
}
impl PartialEq<HealthPlanCostSharingPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanCostSharingPropertyIriOrLabel) -> bool {
		*self == HealthPlanCostSharingPropertyIri
			|| *self == HEALTH_PLAN_COST_SHARING_PROPERTY_LABEL
	}
}
