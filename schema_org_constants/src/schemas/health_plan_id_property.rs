/// <https://schema.org/healthPlanId>
pub const HEALTH_PLAN_ID_PROPERTY_IRI_HTTP: &str = "http://schema.org/healthPlanId";
/// <https://schema.org/healthPlanId>
pub const HEALTH_PLAN_ID_PROPERTY_IRI_HTTPS: &str = "https://schema.org/healthPlanId";
/// <https://schema.org/healthPlanId>
pub const HEALTH_PLAN_ID_PROPERTY_LABEL: &str = "healthPlanId";
pub struct HealthPlanIdPropertyIri;
impl PartialEq<&str> for HealthPlanIdPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_ID_PROPERTY_IRI_HTTP || *other == HEALTH_PLAN_ID_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanIdPropertyIri> for &str {
	fn eq(&self, other: &HealthPlanIdPropertyIri) -> bool {
		*self == HEALTH_PLAN_ID_PROPERTY_IRI_HTTP || *self == HEALTH_PLAN_ID_PROPERTY_IRI_HTTPS
	}
}
pub struct HealthPlanIdPropertyIriOrLabel;
impl PartialEq<&str> for HealthPlanIdPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanIdPropertyIri || *other == HEALTH_PLAN_ID_PROPERTY_LABEL
	}
}
impl PartialEq<HealthPlanIdPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanIdPropertyIriOrLabel) -> bool {
		*self == HealthPlanIdPropertyIri || *self == HEALTH_PLAN_ID_PROPERTY_LABEL
	}
}
