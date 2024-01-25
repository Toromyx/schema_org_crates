/// <https://schema.org/TransitMap>
pub const TRANSIT_MAP_IRI_HTTP: &str = "http://schema.org/TransitMap";
/// <https://schema.org/TransitMap>
pub const TRANSIT_MAP_IRI_HTTPS: &str = "https://schema.org/TransitMap";
/// <https://schema.org/TransitMap>
pub const TRANSIT_MAP_LABEL: &str = "TransitMap";
pub struct TransitMapIri;
impl PartialEq<&str> for TransitMapIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRANSIT_MAP_IRI_HTTP || *other == TRANSIT_MAP_IRI_HTTPS
	}
}
impl PartialEq<TransitMapIri> for &str {
	fn eq(&self, other: &TransitMapIri) -> bool {
		*self == TRANSIT_MAP_IRI_HTTP || *self == TRANSIT_MAP_IRI_HTTPS
	}
}
pub struct TransitMapIriOrLabel;
impl PartialEq<&str> for TransitMapIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TransitMapIri || *other == TRANSIT_MAP_LABEL
	}
}
impl PartialEq<TransitMapIriOrLabel> for &str {
	fn eq(&self, other: &TransitMapIriOrLabel) -> bool {
		*self == TransitMapIri || *self == TRANSIT_MAP_LABEL
	}
}
