/// <https://schema.org/arrivalBusStop>
pub const ARRIVAL_BUS_STOP_PROPERTY_IRI_HTTP: &str = "http://schema.org/arrivalBusStop";
/// <https://schema.org/arrivalBusStop>
pub const ARRIVAL_BUS_STOP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/arrivalBusStop";
/// <https://schema.org/arrivalBusStop>
pub const ARRIVAL_BUS_STOP_PROPERTY_LABEL: &str = "arrivalBusStop";
pub struct ArrivalBusStopPropertyIri;
impl PartialEq<&str> for ArrivalBusStopPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARRIVAL_BUS_STOP_PROPERTY_IRI_HTTP
			|| *other == ARRIVAL_BUS_STOP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArrivalBusStopPropertyIri> for &str {
	fn eq(&self, other: &ArrivalBusStopPropertyIri) -> bool {
		*self == ARRIVAL_BUS_STOP_PROPERTY_IRI_HTTP || *self == ARRIVAL_BUS_STOP_PROPERTY_IRI_HTTPS
	}
}
pub struct ArrivalBusStopPropertyIriOrLabel;
impl PartialEq<&str> for ArrivalBusStopPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArrivalBusStopPropertyIri || *other == ARRIVAL_BUS_STOP_PROPERTY_LABEL
	}
}
impl PartialEq<ArrivalBusStopPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArrivalBusStopPropertyIriOrLabel) -> bool {
		*self == ArrivalBusStopPropertyIri || *self == ARRIVAL_BUS_STOP_PROPERTY_LABEL
	}
}
