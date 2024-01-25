/// <https://schema.org/BoatTrip>
pub const BOAT_TRIP_IRI_HTTP: &str = "http://schema.org/BoatTrip";
/// <https://schema.org/BoatTrip>
pub const BOAT_TRIP_IRI_HTTPS: &str = "https://schema.org/BoatTrip";
/// <https://schema.org/BoatTrip>
pub const BOAT_TRIP_LABEL: &str = "BoatTrip";
pub struct BoatTripIri;
impl PartialEq<&str> for BoatTripIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOAT_TRIP_IRI_HTTP || *other == BOAT_TRIP_IRI_HTTPS
	}
}
impl PartialEq<BoatTripIri> for &str {
	fn eq(&self, other: &BoatTripIri) -> bool {
		*self == BOAT_TRIP_IRI_HTTP || *self == BOAT_TRIP_IRI_HTTPS
	}
}
pub struct BoatTripIriOrLabel;
impl PartialEq<&str> for BoatTripIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BoatTripIri || *other == BOAT_TRIP_LABEL
	}
}
impl PartialEq<BoatTripIriOrLabel> for &str {
	fn eq(&self, other: &BoatTripIriOrLabel) -> bool {
		*self == BoatTripIri || *self == BOAT_TRIP_LABEL
	}
}
