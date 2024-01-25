/// <https://schema.org/OccupationalExperienceRequirements>
pub const OCCUPATIONAL_EXPERIENCE_REQUIREMENTS_IRI_HTTP: &str =
	"http://schema.org/OccupationalExperienceRequirements";
/// <https://schema.org/OccupationalExperienceRequirements>
pub const OCCUPATIONAL_EXPERIENCE_REQUIREMENTS_IRI_HTTPS: &str =
	"https://schema.org/OccupationalExperienceRequirements";
/// <https://schema.org/OccupationalExperienceRequirements>
pub const OCCUPATIONAL_EXPERIENCE_REQUIREMENTS_LABEL: &str = "OccupationalExperienceRequirements";
pub struct OccupationalExperienceRequirementsIri;
impl PartialEq<&str> for OccupationalExperienceRequirementsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OCCUPATIONAL_EXPERIENCE_REQUIREMENTS_IRI_HTTP
			|| *other == OCCUPATIONAL_EXPERIENCE_REQUIREMENTS_IRI_HTTPS
	}
}
impl PartialEq<OccupationalExperienceRequirementsIri> for &str {
	fn eq(&self, other: &OccupationalExperienceRequirementsIri) -> bool {
		*self == OCCUPATIONAL_EXPERIENCE_REQUIREMENTS_IRI_HTTP
			|| *self == OCCUPATIONAL_EXPERIENCE_REQUIREMENTS_IRI_HTTPS
	}
}
pub struct OccupationalExperienceRequirementsIriOrLabel;
impl PartialEq<&str> for OccupationalExperienceRequirementsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OccupationalExperienceRequirementsIri
			|| *other == OCCUPATIONAL_EXPERIENCE_REQUIREMENTS_LABEL
	}
}
impl PartialEq<OccupationalExperienceRequirementsIriOrLabel> for &str {
	fn eq(&self, other: &OccupationalExperienceRequirementsIriOrLabel) -> bool {
		*self == OccupationalExperienceRequirementsIri
			|| *self == OCCUPATIONAL_EXPERIENCE_REQUIREMENTS_LABEL
	}
}
