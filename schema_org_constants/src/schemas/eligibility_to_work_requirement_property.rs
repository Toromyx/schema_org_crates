/// <https://schema.org/eligibilityToWorkRequirement>
pub const ELIGIBILITY_TO_WORK_REQUIREMENT_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/eligibilityToWorkRequirement";
/// <https://schema.org/eligibilityToWorkRequirement>
pub const ELIGIBILITY_TO_WORK_REQUIREMENT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/eligibilityToWorkRequirement";
/// <https://schema.org/eligibilityToWorkRequirement>
pub const ELIGIBILITY_TO_WORK_REQUIREMENT_PROPERTY_LABEL: &str = "eligibilityToWorkRequirement";
pub struct EligibilityToWorkRequirementPropertyIri;
impl PartialEq<&str> for EligibilityToWorkRequirementPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ELIGIBILITY_TO_WORK_REQUIREMENT_PROPERTY_IRI_HTTP
			|| *other == ELIGIBILITY_TO_WORK_REQUIREMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EligibilityToWorkRequirementPropertyIri> for &str {
	fn eq(&self, other: &EligibilityToWorkRequirementPropertyIri) -> bool {
		*self == ELIGIBILITY_TO_WORK_REQUIREMENT_PROPERTY_IRI_HTTP
			|| *self == ELIGIBILITY_TO_WORK_REQUIREMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct EligibilityToWorkRequirementPropertyIriOrLabel;
impl PartialEq<&str> for EligibilityToWorkRequirementPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EligibilityToWorkRequirementPropertyIri
			|| *other == ELIGIBILITY_TO_WORK_REQUIREMENT_PROPERTY_LABEL
	}
}
impl PartialEq<EligibilityToWorkRequirementPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EligibilityToWorkRequirementPropertyIriOrLabel) -> bool {
		*self == EligibilityToWorkRequirementPropertyIri
			|| *self == ELIGIBILITY_TO_WORK_REQUIREMENT_PROPERTY_LABEL
	}
}
