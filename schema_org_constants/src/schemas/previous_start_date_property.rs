/// <https://schema.org/previousStartDate>
pub const PREVIOUS_START_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/previousStartDate";
/// <https://schema.org/previousStartDate>
pub const PREVIOUS_START_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/previousStartDate";
/// <https://schema.org/previousStartDate>
pub const PREVIOUS_START_DATE_PROPERTY_LABEL: &str = "previousStartDate";
pub struct PreviousStartDatePropertyIri;
impl PartialEq<&str> for PreviousStartDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PREVIOUS_START_DATE_PROPERTY_IRI_HTTP
			|| *other == PREVIOUS_START_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PreviousStartDatePropertyIri> for &str {
	fn eq(&self, other: &PreviousStartDatePropertyIri) -> bool {
		*self == PREVIOUS_START_DATE_PROPERTY_IRI_HTTP
			|| *self == PREVIOUS_START_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct PreviousStartDatePropertyIriOrLabel;
impl PartialEq<&str> for PreviousStartDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PreviousStartDatePropertyIri || *other == PREVIOUS_START_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<PreviousStartDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PreviousStartDatePropertyIriOrLabel) -> bool {
		*self == PreviousStartDatePropertyIri || *self == PREVIOUS_START_DATE_PROPERTY_LABEL
	}
}
