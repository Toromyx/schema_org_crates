/// <https://schema.org/healthPlanMarketingUrl>
pub const HEALTH_PLAN_MARKETING_URL_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/healthPlanMarketingUrl";
/// <https://schema.org/healthPlanMarketingUrl>
pub const HEALTH_PLAN_MARKETING_URL_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/healthPlanMarketingUrl";
/// <https://schema.org/healthPlanMarketingUrl>
pub const HEALTH_PLAN_MARKETING_URL_PROPERTY_LABEL: &str = "healthPlanMarketingUrl";
pub struct HealthPlanMarketingUrlPropertyIri;
impl PartialEq<&str> for HealthPlanMarketingUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_MARKETING_URL_PROPERTY_IRI_HTTP
			|| *other == HEALTH_PLAN_MARKETING_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanMarketingUrlPropertyIri> for &str {
	fn eq(&self, other: &HealthPlanMarketingUrlPropertyIri) -> bool {
		*self == HEALTH_PLAN_MARKETING_URL_PROPERTY_IRI_HTTP
			|| *self == HEALTH_PLAN_MARKETING_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct HealthPlanMarketingUrlPropertyIriOrLabel;
impl PartialEq<&str> for HealthPlanMarketingUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanMarketingUrlPropertyIri
			|| *other == HEALTH_PLAN_MARKETING_URL_PROPERTY_LABEL
	}
}
impl PartialEq<HealthPlanMarketingUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanMarketingUrlPropertyIriOrLabel) -> bool {
		*self == HealthPlanMarketingUrlPropertyIri
			|| *self == HEALTH_PLAN_MARKETING_URL_PROPERTY_LABEL
	}
}
