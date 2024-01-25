/// <https://schema.org/usesHealthPlanIdStandard>
pub const USES_HEALTH_PLAN_ID_STANDARD_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/usesHealthPlanIdStandard";
/// <https://schema.org/usesHealthPlanIdStandard>
pub const USES_HEALTH_PLAN_ID_STANDARD_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/usesHealthPlanIdStandard";
/// <https://schema.org/usesHealthPlanIdStandard>
pub const USES_HEALTH_PLAN_ID_STANDARD_PROPERTY_LABEL: &str = "usesHealthPlanIdStandard";
pub struct UsesHealthPlanIdStandardPropertyIri;
impl PartialEq<&str> for UsesHealthPlanIdStandardPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USES_HEALTH_PLAN_ID_STANDARD_PROPERTY_IRI_HTTP
			|| *other == USES_HEALTH_PLAN_ID_STANDARD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UsesHealthPlanIdStandardPropertyIri> for &str {
	fn eq(&self, other: &UsesHealthPlanIdStandardPropertyIri) -> bool {
		*self == USES_HEALTH_PLAN_ID_STANDARD_PROPERTY_IRI_HTTP
			|| *self == USES_HEALTH_PLAN_ID_STANDARD_PROPERTY_IRI_HTTPS
	}
}
pub struct UsesHealthPlanIdStandardPropertyIriOrLabel;
impl PartialEq<&str> for UsesHealthPlanIdStandardPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UsesHealthPlanIdStandardPropertyIri
			|| *other == USES_HEALTH_PLAN_ID_STANDARD_PROPERTY_LABEL
	}
}
impl PartialEq<UsesHealthPlanIdStandardPropertyIriOrLabel> for &str {
	fn eq(&self, other: &UsesHealthPlanIdStandardPropertyIriOrLabel) -> bool {
		*self == UsesHealthPlanIdStandardPropertyIri
			|| *self == USES_HEALTH_PLAN_ID_STANDARD_PROPERTY_LABEL
	}
}
