/// <https://schema.org/includesHealthPlanFormulary>
pub const INCLUDES_HEALTH_PLAN_FORMULARY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/includesHealthPlanFormulary";
/// <https://schema.org/includesHealthPlanFormulary>
pub const INCLUDES_HEALTH_PLAN_FORMULARY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/includesHealthPlanFormulary";
/// <https://schema.org/includesHealthPlanFormulary>
pub const INCLUDES_HEALTH_PLAN_FORMULARY_PROPERTY_LABEL: &str = "includesHealthPlanFormulary";
pub struct IncludesHealthPlanFormularyPropertyIri;
impl PartialEq<&str> for IncludesHealthPlanFormularyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INCLUDES_HEALTH_PLAN_FORMULARY_PROPERTY_IRI_HTTP
			|| *other == INCLUDES_HEALTH_PLAN_FORMULARY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IncludesHealthPlanFormularyPropertyIri> for &str {
	fn eq(&self, other: &IncludesHealthPlanFormularyPropertyIri) -> bool {
		*self == INCLUDES_HEALTH_PLAN_FORMULARY_PROPERTY_IRI_HTTP
			|| *self == INCLUDES_HEALTH_PLAN_FORMULARY_PROPERTY_IRI_HTTPS
	}
}
pub struct IncludesHealthPlanFormularyPropertyIriOrLabel;
impl PartialEq<&str> for IncludesHealthPlanFormularyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IncludesHealthPlanFormularyPropertyIri
			|| *other == INCLUDES_HEALTH_PLAN_FORMULARY_PROPERTY_LABEL
	}
}
impl PartialEq<IncludesHealthPlanFormularyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IncludesHealthPlanFormularyPropertyIriOrLabel) -> bool {
		*self == IncludesHealthPlanFormularyPropertyIri
			|| *self == INCLUDES_HEALTH_PLAN_FORMULARY_PROPERTY_LABEL
	}
}
