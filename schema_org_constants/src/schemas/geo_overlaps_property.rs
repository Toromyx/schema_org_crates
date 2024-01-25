/// <https://schema.org/geoOverlaps>
pub const GEO_OVERLAPS_PROPERTY_IRI_HTTP: &str = "http://schema.org/geoOverlaps";
/// <https://schema.org/geoOverlaps>
pub const GEO_OVERLAPS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/geoOverlaps";
/// <https://schema.org/geoOverlaps>
pub const GEO_OVERLAPS_PROPERTY_LABEL: &str = "geoOverlaps";
pub struct GeoOverlapsPropertyIri;
impl PartialEq<&str> for GeoOverlapsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_OVERLAPS_PROPERTY_IRI_HTTP || *other == GEO_OVERLAPS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GeoOverlapsPropertyIri> for &str {
	fn eq(&self, other: &GeoOverlapsPropertyIri) -> bool {
		*self == GEO_OVERLAPS_PROPERTY_IRI_HTTP || *self == GEO_OVERLAPS_PROPERTY_IRI_HTTPS
	}
}
pub struct GeoOverlapsPropertyIriOrLabel;
impl PartialEq<&str> for GeoOverlapsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoOverlapsPropertyIri || *other == GEO_OVERLAPS_PROPERTY_LABEL
	}
}
impl PartialEq<GeoOverlapsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GeoOverlapsPropertyIriOrLabel) -> bool {
		*self == GeoOverlapsPropertyIri || *self == GEO_OVERLAPS_PROPERTY_LABEL
	}
}
