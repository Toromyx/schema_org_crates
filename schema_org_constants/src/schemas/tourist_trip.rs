/// <https://schema.org/TouristTrip>
pub const TOURIST_TRIP_IRI_HTTP: &str = "http://schema.org/TouristTrip";
/// <https://schema.org/TouristTrip>
pub const TOURIST_TRIP_IRI_HTTPS: &str = "https://schema.org/TouristTrip";
/// <https://schema.org/TouristTrip>
pub const TOURIST_TRIP_LABEL: &str = "TouristTrip";
pub struct TouristTripIri;
impl PartialEq<&str> for TouristTripIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOURIST_TRIP_IRI_HTTP || *other == TOURIST_TRIP_IRI_HTTPS
	}
}
impl PartialEq<TouristTripIri> for &str {
	fn eq(&self, other: &TouristTripIri) -> bool {
		*self == TOURIST_TRIP_IRI_HTTP || *self == TOURIST_TRIP_IRI_HTTPS
	}
}
pub struct TouristTripIriOrLabel;
impl PartialEq<&str> for TouristTripIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TouristTripIri || *other == TOURIST_TRIP_LABEL
	}
}
impl PartialEq<TouristTripIriOrLabel> for &str {
	fn eq(&self, other: &TouristTripIriOrLabel) -> bool {
		*self == TouristTripIri || *self == TOURIST_TRIP_LABEL
	}
}
