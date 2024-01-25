/// <https://schema.org/tripOrigin>
pub const TRIP_ORIGIN_PROPERTY_IRI_HTTP: &str = "http://schema.org/tripOrigin";
/// <https://schema.org/tripOrigin>
pub const TRIP_ORIGIN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/tripOrigin";
/// <https://schema.org/tripOrigin>
pub const TRIP_ORIGIN_PROPERTY_LABEL: &str = "tripOrigin";
pub struct TripOriginPropertyIri;
impl PartialEq<&str> for TripOriginPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRIP_ORIGIN_PROPERTY_IRI_HTTP || *other == TRIP_ORIGIN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TripOriginPropertyIri> for &str {
	fn eq(&self, other: &TripOriginPropertyIri) -> bool {
		*self == TRIP_ORIGIN_PROPERTY_IRI_HTTP || *self == TRIP_ORIGIN_PROPERTY_IRI_HTTPS
	}
}
pub struct TripOriginPropertyIriOrLabel;
impl PartialEq<&str> for TripOriginPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TripOriginPropertyIri || *other == TRIP_ORIGIN_PROPERTY_LABEL
	}
}
impl PartialEq<TripOriginPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TripOriginPropertyIriOrLabel) -> bool {
		*self == TripOriginPropertyIri || *self == TRIP_ORIGIN_PROPERTY_LABEL
	}
}
