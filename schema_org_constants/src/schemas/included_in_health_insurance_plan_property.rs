/// <https://schema.org/includedInHealthInsurancePlan>
pub const INCLUDED_IN_HEALTH_INSURANCE_PLAN_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/includedInHealthInsurancePlan";
/// <https://schema.org/includedInHealthInsurancePlan>
pub const INCLUDED_IN_HEALTH_INSURANCE_PLAN_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/includedInHealthInsurancePlan";
/// <https://schema.org/includedInHealthInsurancePlan>
pub const INCLUDED_IN_HEALTH_INSURANCE_PLAN_PROPERTY_LABEL: &str = "includedInHealthInsurancePlan";
pub struct IncludedInHealthInsurancePlanPropertyIri;
impl PartialEq<&str> for IncludedInHealthInsurancePlanPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INCLUDED_IN_HEALTH_INSURANCE_PLAN_PROPERTY_IRI_HTTP
			|| *other == INCLUDED_IN_HEALTH_INSURANCE_PLAN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IncludedInHealthInsurancePlanPropertyIri> for &str {
	fn eq(&self, other: &IncludedInHealthInsurancePlanPropertyIri) -> bool {
		*self == INCLUDED_IN_HEALTH_INSURANCE_PLAN_PROPERTY_IRI_HTTP
			|| *self == INCLUDED_IN_HEALTH_INSURANCE_PLAN_PROPERTY_IRI_HTTPS
	}
}
pub struct IncludedInHealthInsurancePlanPropertyIriOrLabel;
impl PartialEq<&str> for IncludedInHealthInsurancePlanPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IncludedInHealthInsurancePlanPropertyIri
			|| *other == INCLUDED_IN_HEALTH_INSURANCE_PLAN_PROPERTY_LABEL
	}
}
impl PartialEq<IncludedInHealthInsurancePlanPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IncludedInHealthInsurancePlanPropertyIriOrLabel) -> bool {
		*self == IncludedInHealthInsurancePlanPropertyIri
			|| *self == INCLUDED_IN_HEALTH_INSURANCE_PLAN_PROPERTY_LABEL
	}
}
