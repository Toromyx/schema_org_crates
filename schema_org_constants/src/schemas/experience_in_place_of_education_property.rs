/// <https://schema.org/experienceInPlaceOfEducation>
pub const EXPERIENCE_IN_PLACE_OF_EDUCATION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/experienceInPlaceOfEducation";
/// <https://schema.org/experienceInPlaceOfEducation>
pub const EXPERIENCE_IN_PLACE_OF_EDUCATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/experienceInPlaceOfEducation";
/// <https://schema.org/experienceInPlaceOfEducation>
pub const EXPERIENCE_IN_PLACE_OF_EDUCATION_PROPERTY_LABEL: &str = "experienceInPlaceOfEducation";
pub struct ExperienceInPlaceOfEducationPropertyIri;
impl PartialEq<&str> for ExperienceInPlaceOfEducationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXPERIENCE_IN_PLACE_OF_EDUCATION_PROPERTY_IRI_HTTP
			|| *other == EXPERIENCE_IN_PLACE_OF_EDUCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExperienceInPlaceOfEducationPropertyIri> for &str {
	fn eq(&self, other: &ExperienceInPlaceOfEducationPropertyIri) -> bool {
		*self == EXPERIENCE_IN_PLACE_OF_EDUCATION_PROPERTY_IRI_HTTP
			|| *self == EXPERIENCE_IN_PLACE_OF_EDUCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct ExperienceInPlaceOfEducationPropertyIriOrLabel;
impl PartialEq<&str> for ExperienceInPlaceOfEducationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExperienceInPlaceOfEducationPropertyIri
			|| *other == EXPERIENCE_IN_PLACE_OF_EDUCATION_PROPERTY_LABEL
	}
}
impl PartialEq<ExperienceInPlaceOfEducationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExperienceInPlaceOfEducationPropertyIriOrLabel) -> bool {
		*self == ExperienceInPlaceOfEducationPropertyIri
			|| *self == EXPERIENCE_IN_PLACE_OF_EDUCATION_PROPERTY_LABEL
	}
}
