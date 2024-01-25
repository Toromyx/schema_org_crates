/// <https://schema.org/Airport>
pub const AIRPORT_IRI_HTTP: &str = "http://schema.org/Airport";
/// <https://schema.org/Airport>
pub const AIRPORT_IRI_HTTPS: &str = "https://schema.org/Airport";
/// <https://schema.org/Airport>
pub const AIRPORT_LABEL: &str = "Airport";
pub struct AirportIri;
impl PartialEq<&str> for AirportIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AIRPORT_IRI_HTTP || *other == AIRPORT_IRI_HTTPS
	}
}
impl PartialEq<AirportIri> for &str {
	fn eq(&self, other: &AirportIri) -> bool {
		*self == AIRPORT_IRI_HTTP || *self == AIRPORT_IRI_HTTPS
	}
}
pub struct AirportIriOrLabel;
impl PartialEq<&str> for AirportIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AirportIri || *other == AIRPORT_LABEL
	}
}
impl PartialEq<AirportIriOrLabel> for &str {
	fn eq(&self, other: &AirportIriOrLabel) -> bool {
		*self == AirportIri || *self == AIRPORT_LABEL
	}
}
