/// <https://schema.org/experienceRequirements>
pub const EXPERIENCE_REQUIREMENTS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/experienceRequirements";
/// <https://schema.org/experienceRequirements>
pub const EXPERIENCE_REQUIREMENTS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/experienceRequirements";
/// <https://schema.org/experienceRequirements>
pub const EXPERIENCE_REQUIREMENTS_PROPERTY_LABEL: &str = "experienceRequirements";
pub struct ExperienceRequirementsPropertyIri;
impl PartialEq<&str> for ExperienceRequirementsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXPERIENCE_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *other == EXPERIENCE_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExperienceRequirementsPropertyIri> for &str {
	fn eq(&self, other: &ExperienceRequirementsPropertyIri) -> bool {
		*self == EXPERIENCE_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *self == EXPERIENCE_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct ExperienceRequirementsPropertyIriOrLabel;
impl PartialEq<&str> for ExperienceRequirementsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExperienceRequirementsPropertyIri
			|| *other == EXPERIENCE_REQUIREMENTS_PROPERTY_LABEL
	}
}
impl PartialEq<ExperienceRequirementsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExperienceRequirementsPropertyIriOrLabel) -> bool {
		*self == ExperienceRequirementsPropertyIri
			|| *self == EXPERIENCE_REQUIREMENTS_PROPERTY_LABEL
	}
}
