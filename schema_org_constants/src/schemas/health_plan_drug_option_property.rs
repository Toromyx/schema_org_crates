/// <https://schema.org/healthPlanDrugOption>
pub const HEALTH_PLAN_DRUG_OPTION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/healthPlanDrugOption";
/// <https://schema.org/healthPlanDrugOption>
pub const HEALTH_PLAN_DRUG_OPTION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/healthPlanDrugOption";
/// <https://schema.org/healthPlanDrugOption>
pub const HEALTH_PLAN_DRUG_OPTION_PROPERTY_LABEL: &str = "healthPlanDrugOption";
pub struct HealthPlanDrugOptionPropertyIri;
impl PartialEq<&str> for HealthPlanDrugOptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_DRUG_OPTION_PROPERTY_IRI_HTTP
			|| *other == HEALTH_PLAN_DRUG_OPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanDrugOptionPropertyIri> for &str {
	fn eq(&self, other: &HealthPlanDrugOptionPropertyIri) -> bool {
		*self == HEALTH_PLAN_DRUG_OPTION_PROPERTY_IRI_HTTP
			|| *self == HEALTH_PLAN_DRUG_OPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct HealthPlanDrugOptionPropertyIriOrLabel;
impl PartialEq<&str> for HealthPlanDrugOptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanDrugOptionPropertyIri
			|| *other == HEALTH_PLAN_DRUG_OPTION_PROPERTY_LABEL
	}
}
impl PartialEq<HealthPlanDrugOptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanDrugOptionPropertyIriOrLabel) -> bool {
		*self == HealthPlanDrugOptionPropertyIri || *self == HEALTH_PLAN_DRUG_OPTION_PROPERTY_LABEL
	}
}
