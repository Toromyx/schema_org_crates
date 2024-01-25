/// <https://schema.org/arrivalAirport>
pub const ARRIVAL_AIRPORT_PROPERTY_IRI_HTTP: &str = "http://schema.org/arrivalAirport";
/// <https://schema.org/arrivalAirport>
pub const ARRIVAL_AIRPORT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/arrivalAirport";
/// <https://schema.org/arrivalAirport>
pub const ARRIVAL_AIRPORT_PROPERTY_LABEL: &str = "arrivalAirport";
pub struct ArrivalAirportPropertyIri;
impl PartialEq<&str> for ArrivalAirportPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARRIVAL_AIRPORT_PROPERTY_IRI_HTTP || *other == ARRIVAL_AIRPORT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArrivalAirportPropertyIri> for &str {
	fn eq(&self, other: &ArrivalAirportPropertyIri) -> bool {
		*self == ARRIVAL_AIRPORT_PROPERTY_IRI_HTTP || *self == ARRIVAL_AIRPORT_PROPERTY_IRI_HTTPS
	}
}
pub struct ArrivalAirportPropertyIriOrLabel;
impl PartialEq<&str> for ArrivalAirportPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArrivalAirportPropertyIri || *other == ARRIVAL_AIRPORT_PROPERTY_LABEL
	}
}
impl PartialEq<ArrivalAirportPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArrivalAirportPropertyIriOrLabel) -> bool {
		*self == ArrivalAirportPropertyIri || *self == ARRIVAL_AIRPORT_PROPERTY_LABEL
	}
}
