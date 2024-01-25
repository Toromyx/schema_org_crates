/// <https://schema.org/FloorPlan>
pub const FLOOR_PLAN_IRI_HTTP: &str = "http://schema.org/FloorPlan";
/// <https://schema.org/FloorPlan>
pub const FLOOR_PLAN_IRI_HTTPS: &str = "https://schema.org/FloorPlan";
/// <https://schema.org/FloorPlan>
pub const FLOOR_PLAN_LABEL: &str = "FloorPlan";
pub struct FloorPlanIri;
impl PartialEq<&str> for FloorPlanIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FLOOR_PLAN_IRI_HTTP || *other == FLOOR_PLAN_IRI_HTTPS
	}
}
impl PartialEq<FloorPlanIri> for &str {
	fn eq(&self, other: &FloorPlanIri) -> bool {
		*self == FLOOR_PLAN_IRI_HTTP || *self == FLOOR_PLAN_IRI_HTTPS
	}
}
pub struct FloorPlanIriOrLabel;
impl PartialEq<&str> for FloorPlanIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FloorPlanIri || *other == FLOOR_PLAN_LABEL
	}
}
impl PartialEq<FloorPlanIriOrLabel> for &str {
	fn eq(&self, other: &FloorPlanIriOrLabel) -> bool {
		*self == FloorPlanIri || *self == FLOOR_PLAN_LABEL
	}
}
