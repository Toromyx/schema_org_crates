/// <https://schema.org/geoIntersects>
pub const GEO_INTERSECTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/geoIntersects";
/// <https://schema.org/geoIntersects>
pub const GEO_INTERSECTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/geoIntersects";
/// <https://schema.org/geoIntersects>
pub const GEO_INTERSECTS_PROPERTY_LABEL: &str = "geoIntersects";
pub struct GeoIntersectsPropertyIri;
impl PartialEq<&str> for GeoIntersectsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_INTERSECTS_PROPERTY_IRI_HTTP || *other == GEO_INTERSECTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GeoIntersectsPropertyIri> for &str {
	fn eq(&self, other: &GeoIntersectsPropertyIri) -> bool {
		*self == GEO_INTERSECTS_PROPERTY_IRI_HTTP || *self == GEO_INTERSECTS_PROPERTY_IRI_HTTPS
	}
}
pub struct GeoIntersectsPropertyIriOrLabel;
impl PartialEq<&str> for GeoIntersectsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoIntersectsPropertyIri || *other == GEO_INTERSECTS_PROPERTY_LABEL
	}
}
impl PartialEq<GeoIntersectsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GeoIntersectsPropertyIriOrLabel) -> bool {
		*self == GeoIntersectsPropertyIri || *self == GEO_INTERSECTS_PROPERTY_LABEL
	}
}
