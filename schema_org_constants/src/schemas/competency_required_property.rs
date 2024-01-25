/// <https://schema.org/competencyRequired>
pub const COMPETENCY_REQUIRED_PROPERTY_IRI_HTTP: &str = "http://schema.org/competencyRequired";
/// <https://schema.org/competencyRequired>
pub const COMPETENCY_REQUIRED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/competencyRequired";
/// <https://schema.org/competencyRequired>
pub const COMPETENCY_REQUIRED_PROPERTY_LABEL: &str = "competencyRequired";
pub struct CompetencyRequiredPropertyIri;
impl PartialEq<&str> for CompetencyRequiredPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMPETENCY_REQUIRED_PROPERTY_IRI_HTTP
			|| *other == COMPETENCY_REQUIRED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CompetencyRequiredPropertyIri> for &str {
	fn eq(&self, other: &CompetencyRequiredPropertyIri) -> bool {
		*self == COMPETENCY_REQUIRED_PROPERTY_IRI_HTTP
			|| *self == COMPETENCY_REQUIRED_PROPERTY_IRI_HTTPS
	}
}
pub struct CompetencyRequiredPropertyIriOrLabel;
impl PartialEq<&str> for CompetencyRequiredPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CompetencyRequiredPropertyIri || *other == COMPETENCY_REQUIRED_PROPERTY_LABEL
	}
}
impl PartialEq<CompetencyRequiredPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CompetencyRequiredPropertyIriOrLabel) -> bool {
		*self == CompetencyRequiredPropertyIri || *self == COMPETENCY_REQUIRED_PROPERTY_LABEL
	}
}
