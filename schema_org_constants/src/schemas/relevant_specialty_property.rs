/// <https://schema.org/relevantSpecialty>
pub const RELEVANT_SPECIALTY_PROPERTY_IRI_HTTP: &str = "http://schema.org/relevantSpecialty";
/// <https://schema.org/relevantSpecialty>
pub const RELEVANT_SPECIALTY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/relevantSpecialty";
/// <https://schema.org/relevantSpecialty>
pub const RELEVANT_SPECIALTY_PROPERTY_LABEL: &str = "relevantSpecialty";
pub struct RelevantSpecialtyPropertyIri;
impl PartialEq<&str> for RelevantSpecialtyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RELEVANT_SPECIALTY_PROPERTY_IRI_HTTP
			|| *other == RELEVANT_SPECIALTY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RelevantSpecialtyPropertyIri> for &str {
	fn eq(&self, other: &RelevantSpecialtyPropertyIri) -> bool {
		*self == RELEVANT_SPECIALTY_PROPERTY_IRI_HTTP
			|| *self == RELEVANT_SPECIALTY_PROPERTY_IRI_HTTPS
	}
}
pub struct RelevantSpecialtyPropertyIriOrLabel;
impl PartialEq<&str> for RelevantSpecialtyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RelevantSpecialtyPropertyIri || *other == RELEVANT_SPECIALTY_PROPERTY_LABEL
	}
}
impl PartialEq<RelevantSpecialtyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RelevantSpecialtyPropertyIriOrLabel) -> bool {
		*self == RelevantSpecialtyPropertyIri || *self == RELEVANT_SPECIALTY_PROPERTY_LABEL
	}
}
