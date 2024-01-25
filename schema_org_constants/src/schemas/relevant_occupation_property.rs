/// <https://schema.org/relevantOccupation>
pub const RELEVANT_OCCUPATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/relevantOccupation";
/// <https://schema.org/relevantOccupation>
pub const RELEVANT_OCCUPATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/relevantOccupation";
/// <https://schema.org/relevantOccupation>
pub const RELEVANT_OCCUPATION_PROPERTY_LABEL: &str = "relevantOccupation";
pub struct RelevantOccupationPropertyIri;
impl PartialEq<&str> for RelevantOccupationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RELEVANT_OCCUPATION_PROPERTY_IRI_HTTP
			|| *other == RELEVANT_OCCUPATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RelevantOccupationPropertyIri> for &str {
	fn eq(&self, other: &RelevantOccupationPropertyIri) -> bool {
		*self == RELEVANT_OCCUPATION_PROPERTY_IRI_HTTP
			|| *self == RELEVANT_OCCUPATION_PROPERTY_IRI_HTTPS
	}
}
pub struct RelevantOccupationPropertyIriOrLabel;
impl PartialEq<&str> for RelevantOccupationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RelevantOccupationPropertyIri || *other == RELEVANT_OCCUPATION_PROPERTY_LABEL
	}
}
impl PartialEq<RelevantOccupationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RelevantOccupationPropertyIriOrLabel) -> bool {
		*self == RelevantOccupationPropertyIri || *self == RELEVANT_OCCUPATION_PROPERTY_LABEL
	}
}
