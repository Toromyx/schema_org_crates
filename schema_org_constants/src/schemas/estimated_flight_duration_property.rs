/// <https://schema.org/estimatedFlightDuration>
pub const ESTIMATED_FLIGHT_DURATION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/estimatedFlightDuration";
/// <https://schema.org/estimatedFlightDuration>
pub const ESTIMATED_FLIGHT_DURATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/estimatedFlightDuration";
/// <https://schema.org/estimatedFlightDuration>
pub const ESTIMATED_FLIGHT_DURATION_PROPERTY_LABEL: &str = "estimatedFlightDuration";
pub struct EstimatedFlightDurationPropertyIri;
impl PartialEq<&str> for EstimatedFlightDurationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ESTIMATED_FLIGHT_DURATION_PROPERTY_IRI_HTTP
			|| *other == ESTIMATED_FLIGHT_DURATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EstimatedFlightDurationPropertyIri> for &str {
	fn eq(&self, other: &EstimatedFlightDurationPropertyIri) -> bool {
		*self == ESTIMATED_FLIGHT_DURATION_PROPERTY_IRI_HTTP
			|| *self == ESTIMATED_FLIGHT_DURATION_PROPERTY_IRI_HTTPS
	}
}
pub struct EstimatedFlightDurationPropertyIriOrLabel;
impl PartialEq<&str> for EstimatedFlightDurationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EstimatedFlightDurationPropertyIri
			|| *other == ESTIMATED_FLIGHT_DURATION_PROPERTY_LABEL
	}
}
impl PartialEq<EstimatedFlightDurationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EstimatedFlightDurationPropertyIriOrLabel) -> bool {
		*self == EstimatedFlightDurationPropertyIri
			|| *self == ESTIMATED_FLIGHT_DURATION_PROPERTY_LABEL
	}
}
