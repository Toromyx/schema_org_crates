/// <https://schema.org/healthPlanPharmacyCategory>
pub const HEALTH_PLAN_PHARMACY_CATEGORY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/healthPlanPharmacyCategory";
/// <https://schema.org/healthPlanPharmacyCategory>
pub const HEALTH_PLAN_PHARMACY_CATEGORY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/healthPlanPharmacyCategory";
/// <https://schema.org/healthPlanPharmacyCategory>
pub const HEALTH_PLAN_PHARMACY_CATEGORY_PROPERTY_LABEL: &str = "healthPlanPharmacyCategory";
pub struct HealthPlanPharmacyCategoryPropertyIri;
impl PartialEq<&str> for HealthPlanPharmacyCategoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_PLAN_PHARMACY_CATEGORY_PROPERTY_IRI_HTTP
			|| *other == HEALTH_PLAN_PHARMACY_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HealthPlanPharmacyCategoryPropertyIri> for &str {
	fn eq(&self, other: &HealthPlanPharmacyCategoryPropertyIri) -> bool {
		*self == HEALTH_PLAN_PHARMACY_CATEGORY_PROPERTY_IRI_HTTP
			|| *self == HEALTH_PLAN_PHARMACY_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
pub struct HealthPlanPharmacyCategoryPropertyIriOrLabel;
impl PartialEq<&str> for HealthPlanPharmacyCategoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthPlanPharmacyCategoryPropertyIri
			|| *other == HEALTH_PLAN_PHARMACY_CATEGORY_PROPERTY_LABEL
	}
}
impl PartialEq<HealthPlanPharmacyCategoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HealthPlanPharmacyCategoryPropertyIriOrLabel) -> bool {
		*self == HealthPlanPharmacyCategoryPropertyIri
			|| *self == HEALTH_PLAN_PHARMACY_CATEGORY_PROPERTY_LABEL
	}
}
