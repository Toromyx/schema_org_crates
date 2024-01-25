/// <https://schema.org/Observation>
pub const OBSERVATION_IRI_HTTP: &str = "http://schema.org/Observation";
/// <https://schema.org/Observation>
pub const OBSERVATION_IRI_HTTPS: &str = "https://schema.org/Observation";
/// <https://schema.org/Observation>
pub const OBSERVATION_LABEL: &str = "Observation";
pub struct ObservationIri;
impl PartialEq<&str> for ObservationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OBSERVATION_IRI_HTTP || *other == OBSERVATION_IRI_HTTPS
	}
}
impl PartialEq<ObservationIri> for &str {
	fn eq(&self, other: &ObservationIri) -> bool {
		*self == OBSERVATION_IRI_HTTP || *self == OBSERVATION_IRI_HTTPS
	}
}
pub struct ObservationIriOrLabel;
impl PartialEq<&str> for ObservationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ObservationIri || *other == OBSERVATION_LABEL
	}
}
impl PartialEq<ObservationIriOrLabel> for &str {
	fn eq(&self, other: &ObservationIriOrLabel) -> bool {
		*self == ObservationIri || *self == OBSERVATION_LABEL
	}
}
