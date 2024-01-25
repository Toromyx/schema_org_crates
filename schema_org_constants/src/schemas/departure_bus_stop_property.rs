/// <https://schema.org/departureBusStop>
pub const DEPARTURE_BUS_STOP_PROPERTY_IRI_HTTP: &str = "http://schema.org/departureBusStop";
/// <https://schema.org/departureBusStop>
pub const DEPARTURE_BUS_STOP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/departureBusStop";
/// <https://schema.org/departureBusStop>
pub const DEPARTURE_BUS_STOP_PROPERTY_LABEL: &str = "departureBusStop";
pub struct DepartureBusStopPropertyIri;
impl PartialEq<&str> for DepartureBusStopPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEPARTURE_BUS_STOP_PROPERTY_IRI_HTTP
			|| *other == DEPARTURE_BUS_STOP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DepartureBusStopPropertyIri> for &str {
	fn eq(&self, other: &DepartureBusStopPropertyIri) -> bool {
		*self == DEPARTURE_BUS_STOP_PROPERTY_IRI_HTTP
			|| *self == DEPARTURE_BUS_STOP_PROPERTY_IRI_HTTPS
	}
}
pub struct DepartureBusStopPropertyIriOrLabel;
impl PartialEq<&str> for DepartureBusStopPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DepartureBusStopPropertyIri || *other == DEPARTURE_BUS_STOP_PROPERTY_LABEL
	}
}
impl PartialEq<DepartureBusStopPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DepartureBusStopPropertyIriOrLabel) -> bool {
		*self == DepartureBusStopPropertyIri || *self == DEPARTURE_BUS_STOP_PROPERTY_LABEL
	}
}
