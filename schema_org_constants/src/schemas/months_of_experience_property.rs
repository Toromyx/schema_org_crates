/// <https://schema.org/monthsOfExperience>
pub const MONTHS_OF_EXPERIENCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/monthsOfExperience";
/// <https://schema.org/monthsOfExperience>
pub const MONTHS_OF_EXPERIENCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/monthsOfExperience";
/// <https://schema.org/monthsOfExperience>
pub const MONTHS_OF_EXPERIENCE_PROPERTY_LABEL: &str = "monthsOfExperience";
pub struct MonthsOfExperiencePropertyIri;
impl PartialEq<&str> for MonthsOfExperiencePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MONTHS_OF_EXPERIENCE_PROPERTY_IRI_HTTP
			|| *other == MONTHS_OF_EXPERIENCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MonthsOfExperiencePropertyIri> for &str {
	fn eq(&self, other: &MonthsOfExperiencePropertyIri) -> bool {
		*self == MONTHS_OF_EXPERIENCE_PROPERTY_IRI_HTTP
			|| *self == MONTHS_OF_EXPERIENCE_PROPERTY_IRI_HTTPS
	}
}
pub struct MonthsOfExperiencePropertyIriOrLabel;
impl PartialEq<&str> for MonthsOfExperiencePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MonthsOfExperiencePropertyIri || *other == MONTHS_OF_EXPERIENCE_PROPERTY_LABEL
	}
}
impl PartialEq<MonthsOfExperiencePropertyIriOrLabel> for &str {
	fn eq(&self, other: &MonthsOfExperiencePropertyIriOrLabel) -> bool {
		*self == MonthsOfExperiencePropertyIri || *self == MONTHS_OF_EXPERIENCE_PROPERTY_LABEL
	}
}
