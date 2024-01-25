/// <https://schema.org/applicantLocationRequirements>
pub const APPLICANT_LOCATION_REQUIREMENTS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/applicantLocationRequirements";
/// <https://schema.org/applicantLocationRequirements>
pub const APPLICANT_LOCATION_REQUIREMENTS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/applicantLocationRequirements";
/// <https://schema.org/applicantLocationRequirements>
pub const APPLICANT_LOCATION_REQUIREMENTS_PROPERTY_LABEL: &str = "applicantLocationRequirements";
pub struct ApplicantLocationRequirementsPropertyIri;
impl PartialEq<&str> for ApplicantLocationRequirementsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPLICANT_LOCATION_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *other == APPLICANT_LOCATION_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ApplicantLocationRequirementsPropertyIri> for &str {
	fn eq(&self, other: &ApplicantLocationRequirementsPropertyIri) -> bool {
		*self == APPLICANT_LOCATION_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *self == APPLICANT_LOCATION_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct ApplicantLocationRequirementsPropertyIriOrLabel;
impl PartialEq<&str> for ApplicantLocationRequirementsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApplicantLocationRequirementsPropertyIri
			|| *other == APPLICANT_LOCATION_REQUIREMENTS_PROPERTY_LABEL
	}
}
impl PartialEq<ApplicantLocationRequirementsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ApplicantLocationRequirementsPropertyIriOrLabel) -> bool {
		*self == ApplicantLocationRequirementsPropertyIri
			|| *self == APPLICANT_LOCATION_REQUIREMENTS_PROPERTY_LABEL
	}
}
