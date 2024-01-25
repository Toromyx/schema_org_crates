/// <https://schema.org/SeatingMap>
pub const SEATING_MAP_IRI_HTTP: &str = "http://schema.org/SeatingMap";
/// <https://schema.org/SeatingMap>
pub const SEATING_MAP_IRI_HTTPS: &str = "https://schema.org/SeatingMap";
/// <https://schema.org/SeatingMap>
pub const SEATING_MAP_LABEL: &str = "SeatingMap";
pub struct SeatingMapIri;
impl PartialEq<&str> for SeatingMapIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEATING_MAP_IRI_HTTP || *other == SEATING_MAP_IRI_HTTPS
	}
}
impl PartialEq<SeatingMapIri> for &str {
	fn eq(&self, other: &SeatingMapIri) -> bool {
		*self == SEATING_MAP_IRI_HTTP || *self == SEATING_MAP_IRI_HTTPS
	}
}
pub struct SeatingMapIriOrLabel;
impl PartialEq<&str> for SeatingMapIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeatingMapIri || *other == SEATING_MAP_LABEL
	}
}
impl PartialEq<SeatingMapIriOrLabel> for &str {
	fn eq(&self, other: &SeatingMapIriOrLabel) -> bool {
		*self == SeatingMapIri || *self == SEATING_MAP_LABEL
	}
}
