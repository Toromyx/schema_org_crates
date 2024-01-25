/// <https://schema.org/educationalLevel>
pub const EDUCATIONAL_LEVEL_PROPERTY_IRI_HTTP: &str = "http://schema.org/educationalLevel";
/// <https://schema.org/educationalLevel>
pub const EDUCATIONAL_LEVEL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/educationalLevel";
/// <https://schema.org/educationalLevel>
pub const EDUCATIONAL_LEVEL_PROPERTY_LABEL: &str = "educationalLevel";
pub struct EducationalLevelPropertyIri;
impl PartialEq<&str> for EducationalLevelPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDUCATIONAL_LEVEL_PROPERTY_IRI_HTTP
			|| *other == EDUCATIONAL_LEVEL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EducationalLevelPropertyIri> for &str {
	fn eq(&self, other: &EducationalLevelPropertyIri) -> bool {
		*self == EDUCATIONAL_LEVEL_PROPERTY_IRI_HTTP
			|| *self == EDUCATIONAL_LEVEL_PROPERTY_IRI_HTTPS
	}
}
pub struct EducationalLevelPropertyIriOrLabel;
impl PartialEq<&str> for EducationalLevelPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EducationalLevelPropertyIri || *other == EDUCATIONAL_LEVEL_PROPERTY_LABEL
	}
}
impl PartialEq<EducationalLevelPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EducationalLevelPropertyIriOrLabel) -> bool {
		*self == EducationalLevelPropertyIri || *self == EDUCATIONAL_LEVEL_PROPERTY_LABEL
	}
}
