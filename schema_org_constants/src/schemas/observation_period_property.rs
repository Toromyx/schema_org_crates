/// <https://schema.org/observationPeriod>
pub const OBSERVATION_PERIOD_PROPERTY_IRI_HTTP: &str = "http://schema.org/observationPeriod";
/// <https://schema.org/observationPeriod>
pub const OBSERVATION_PERIOD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/observationPeriod";
/// <https://schema.org/observationPeriod>
pub const OBSERVATION_PERIOD_PROPERTY_LABEL: &str = "observationPeriod";
pub struct ObservationPeriodPropertyIri;
impl PartialEq<&str> for ObservationPeriodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OBSERVATION_PERIOD_PROPERTY_IRI_HTTP
			|| *other == OBSERVATION_PERIOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ObservationPeriodPropertyIri> for &str {
	fn eq(&self, other: &ObservationPeriodPropertyIri) -> bool {
		*self == OBSERVATION_PERIOD_PROPERTY_IRI_HTTP
			|| *self == OBSERVATION_PERIOD_PROPERTY_IRI_HTTPS
	}
}
pub struct ObservationPeriodPropertyIriOrLabel;
impl PartialEq<&str> for ObservationPeriodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ObservationPeriodPropertyIri || *other == OBSERVATION_PERIOD_PROPERTY_LABEL
	}
}
impl PartialEq<ObservationPeriodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ObservationPeriodPropertyIriOrLabel) -> bool {
		*self == ObservationPeriodPropertyIri || *self == OBSERVATION_PERIOD_PROPERTY_LABEL
	}
}
