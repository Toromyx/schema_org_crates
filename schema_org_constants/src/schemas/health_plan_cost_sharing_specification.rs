/// <https://schema.org/HealthPlanCostSharingSpecification>
pub const HEALTH_PLAN_COST_SHARING_SPECIFICATION_IRI_HTTP: &str =
	"http://schema.org/HealthPlanCostSharingSpecification";
/// <https://schema.org/HealthPlanCostSharingSpecification>
pub const HEALTH_PLAN_COST_SHARING_SPECIFICATION_IRI_HTTPS: &str =
	"https://schema.org/HealthPlanCostSharingSpecification";
/// <https://schema.org/HealthPlanCostSharingSpecification>
pub const HEALTH_PLAN_COST_SHARING_SPECIFICATION_LABEL: &str = "HealthPlanCostSharingSpecification";
pub struct HealthPlanCostSharingSpecificationIri;
impl PartialEq<&str> for HealthPlanCostSharingSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_COST_SHARING_SPECIFICATION_IRI_HTTP
			|| *other == HEALTH_PLAN_COST_SHARING_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanCostSharingSpecificationIri> for &str {
	fn eq(&self, other: &HealthPlanCostSharingSpecificationIri) -> bool {
		*self == HEALTH_PLAN_COST_SHARING_SPECIFICATION_IRI_HTTP
			|| *self == HEALTH_PLAN_COST_SHARING_SPECIFICATION_IRI_HTTPS
	}
}
pub struct HealthPlanCostSharingSpecificationIriOrLabel;
impl PartialEq<&str> for HealthPlanCostSharingSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanCostSharingSpecificationIri
			|| *other == HEALTH_PLAN_COST_SHARING_SPECIFICATION_LABEL
	}
}
impl PartialEq<HealthPlanCostSharingSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanCostSharingSpecificationIriOrLabel) -> bool {
		*self == HealthPlanCostSharingSpecificationIri
			|| *self == HEALTH_PLAN_COST_SHARING_SPECIFICATION_LABEL
	}
}
