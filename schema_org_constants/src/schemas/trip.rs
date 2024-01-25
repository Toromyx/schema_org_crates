/// <https://schema.org/Trip>
pub const TRIP_IRI_HTTP: &str = "http://schema.org/Trip";
/// <https://schema.org/Trip>
pub const TRIP_IRI_HTTPS: &str = "https://schema.org/Trip";
/// <https://schema.org/Trip>
pub const TRIP_LABEL: &str = "Trip";
pub struct TripIri;
impl PartialEq<&str> for TripIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRIP_IRI_HTTP || *other == TRIP_IRI_HTTPS
	}
}
impl PartialEq<TripIri> for &str {
	fn eq(&self, other: &TripIri) -> bool {
		*self == TRIP_IRI_HTTP || *self == TRIP_IRI_HTTPS
	}
}
pub struct TripIriOrLabel;
impl PartialEq<&str> for TripIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TripIri || *other == TRIP_LABEL
	}
}
impl PartialEq<TripIriOrLabel> for &str {
	fn eq(&self, other: &TripIriOrLabel) -> bool {
		*self == TripIri || *self == TRIP_LABEL
	}
}
