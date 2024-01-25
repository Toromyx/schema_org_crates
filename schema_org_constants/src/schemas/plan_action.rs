/// <https://schema.org/PlanAction>
pub const PLAN_ACTION_IRI_HTTP: &str = "http://schema.org/PlanAction";
/// <https://schema.org/PlanAction>
pub const PLAN_ACTION_IRI_HTTPS: &str = "https://schema.org/PlanAction";
/// <https://schema.org/PlanAction>
pub const PLAN_ACTION_LABEL: &str = "PlanAction";
pub struct PlanActionIri;
impl PartialEq<&str> for PlanActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PLAN_ACTION_IRI_HTTP || *other == PLAN_ACTION_IRI_HTTPS
	}
}
impl PartialEq<PlanActionIri> for &str {
	fn eq(&self, other: &PlanActionIri) -> bool {
		*self == PLAN_ACTION_IRI_HTTP || *self == PLAN_ACTION_IRI_HTTPS
	}
}
pub struct PlanActionIriOrLabel;
impl PartialEq<&str> for PlanActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PlanActionIri || *other == PLAN_ACTION_LABEL
	}
}
impl PartialEq<PlanActionIriOrLabel> for &str {
	fn eq(&self, other: &PlanActionIriOrLabel) -> bool {
		*self == PlanActionIri || *self == PLAN_ACTION_LABEL
	}
}
