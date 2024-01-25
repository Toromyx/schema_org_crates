/// <https://schema.org/proficiencyLevel>
pub const PROFICIENCY_LEVEL_PROPERTY_IRI_HTTP: &str = "http://schema.org/proficiencyLevel";
/// <https://schema.org/proficiencyLevel>
pub const PROFICIENCY_LEVEL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/proficiencyLevel";
/// <https://schema.org/proficiencyLevel>
pub const PROFICIENCY_LEVEL_PROPERTY_LABEL: &str = "proficiencyLevel";
pub struct ProficiencyLevelPropertyIri;
impl PartialEq<&str> for ProficiencyLevelPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROFICIENCY_LEVEL_PROPERTY_IRI_HTTP
			|| *other == PROFICIENCY_LEVEL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProficiencyLevelPropertyIri> for &str {
	fn eq(&self, other: &ProficiencyLevelPropertyIri) -> bool {
		*self == PROFICIENCY_LEVEL_PROPERTY_IRI_HTTP
			|| *self == PROFICIENCY_LEVEL_PROPERTY_IRI_HTTPS
	}
}
pub struct ProficiencyLevelPropertyIriOrLabel;
impl PartialEq<&str> for ProficiencyLevelPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProficiencyLevelPropertyIri || *other == PROFICIENCY_LEVEL_PROPERTY_LABEL
	}
}
impl PartialEq<ProficiencyLevelPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProficiencyLevelPropertyIriOrLabel) -> bool {
		*self == ProficiencyLevelPropertyIri || *self == PROFICIENCY_LEVEL_PROPERTY_LABEL
	}
}
