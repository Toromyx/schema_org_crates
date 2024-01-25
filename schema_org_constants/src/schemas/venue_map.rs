/// <https://schema.org/VenueMap>
pub const VENUE_MAP_IRI_HTTP: &str = "http://schema.org/VenueMap";
/// <https://schema.org/VenueMap>
pub const VENUE_MAP_IRI_HTTPS: &str = "https://schema.org/VenueMap";
/// <https://schema.org/VenueMap>
pub const VENUE_MAP_LABEL: &str = "VenueMap";
pub struct VenueMapIri;
impl PartialEq<&str> for VenueMapIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VENUE_MAP_IRI_HTTP || *other == VENUE_MAP_IRI_HTTPS
	}
}
impl PartialEq<VenueMapIri> for &str {
	fn eq(&self, other: &VenueMapIri) -> bool {
		*self == VENUE_MAP_IRI_HTTP || *self == VENUE_MAP_IRI_HTTPS
	}
}
pub struct VenueMapIriOrLabel;
impl PartialEq<&str> for VenueMapIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VenueMapIri || *other == VENUE_MAP_LABEL
	}
}
impl PartialEq<VenueMapIriOrLabel> for &str {
	fn eq(&self, other: &VenueMapIriOrLabel) -> bool {
		*self == VenueMapIri || *self == VENUE_MAP_LABEL
	}
}
