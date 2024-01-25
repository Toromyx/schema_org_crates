/// <https://schema.org/HealthPlanFormulary>
pub const HEALTH_PLAN_FORMULARY_IRI_HTTP: &str = "http://schema.org/HealthPlanFormulary";
/// <https://schema.org/HealthPlanFormulary>
pub const HEALTH_PLAN_FORMULARY_IRI_HTTPS: &str = "https://schema.org/HealthPlanFormulary";
/// <https://schema.org/HealthPlanFormulary>
pub const HEALTH_PLAN_FORMULARY_LABEL: &str = "HealthPlanFormulary";
pub struct HealthPlanFormularyIri;
impl PartialEq<&str> for HealthPlanFormularyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_FORMULARY_IRI_HTTP || *other == HEALTH_PLAN_FORMULARY_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanFormularyIri> for &str {
	fn eq(&self, other: &HealthPlanFormularyIri) -> bool {
		*self == HEALTH_PLAN_FORMULARY_IRI_HTTP || *self == HEALTH_PLAN_FORMULARY_IRI_HTTPS
	}
}
pub struct HealthPlanFormularyIriOrLabel;
impl PartialEq<&str> for HealthPlanFormularyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanFormularyIri || *other == HEALTH_PLAN_FORMULARY_LABEL
	}
}
impl PartialEq<HealthPlanFormularyIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanFormularyIriOrLabel) -> bool {
		*self == HealthPlanFormularyIri || *self == HEALTH_PLAN_FORMULARY_LABEL
	}
}
