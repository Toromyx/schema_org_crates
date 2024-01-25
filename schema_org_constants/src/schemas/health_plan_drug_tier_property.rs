/// <https://schema.org/healthPlanDrugTier>
pub const HEALTH_PLAN_DRUG_TIER_PROPERTY_IRI_HTTP: &str = "http://schema.org/healthPlanDrugTier";
/// <https://schema.org/healthPlanDrugTier>
pub const HEALTH_PLAN_DRUG_TIER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/healthPlanDrugTier";
/// <https://schema.org/healthPlanDrugTier>
pub const HEALTH_PLAN_DRUG_TIER_PROPERTY_LABEL: &str = "healthPlanDrugTier";
pub struct HealthPlanDrugTierPropertyIri;
impl PartialEq<&str> for HealthPlanDrugTierPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_DRUG_TIER_PROPERTY_IRI_HTTP
			|| *other == HEALTH_PLAN_DRUG_TIER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanDrugTierPropertyIri> for &str {
	fn eq(&self, other: &HealthPlanDrugTierPropertyIri) -> bool {
		*self == HEALTH_PLAN_DRUG_TIER_PROPERTY_IRI_HTTP
			|| *self == HEALTH_PLAN_DRUG_TIER_PROPERTY_IRI_HTTPS
	}
}
pub struct HealthPlanDrugTierPropertyIriOrLabel;
impl PartialEq<&str> for HealthPlanDrugTierPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanDrugTierPropertyIri || *other == HEALTH_PLAN_DRUG_TIER_PROPERTY_LABEL
	}
}
impl PartialEq<HealthPlanDrugTierPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanDrugTierPropertyIriOrLabel) -> bool {
		*self == HealthPlanDrugTierPropertyIri || *self == HEALTH_PLAN_DRUG_TIER_PROPERTY_LABEL
	}
}
