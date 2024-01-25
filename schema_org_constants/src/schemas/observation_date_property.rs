/// <https://schema.org/observationDate>
pub const OBSERVATION_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/observationDate";
/// <https://schema.org/observationDate>
pub const OBSERVATION_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/observationDate";
/// <https://schema.org/observationDate>
pub const OBSERVATION_DATE_PROPERTY_LABEL: &str = "observationDate";
pub struct ObservationDatePropertyIri;
impl PartialEq<&str> for ObservationDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OBSERVATION_DATE_PROPERTY_IRI_HTTP
			|| *other == OBSERVATION_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ObservationDatePropertyIri> for &str {
	fn eq(&self, other: &ObservationDatePropertyIri) -> bool {
		*self == OBSERVATION_DATE_PROPERTY_IRI_HTTP || *self == OBSERVATION_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct ObservationDatePropertyIriOrLabel;
impl PartialEq<&str> for ObservationDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ObservationDatePropertyIri || *other == OBSERVATION_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<ObservationDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ObservationDatePropertyIriOrLabel) -> bool {
		*self == ObservationDatePropertyIri || *self == OBSERVATION_DATE_PROPERTY_LABEL
	}
}
