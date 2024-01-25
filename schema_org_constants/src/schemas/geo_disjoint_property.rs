/// <https://schema.org/geoDisjoint>
pub const GEO_DISJOINT_PROPERTY_IRI_HTTP: &str = "http://schema.org/geoDisjoint";
/// <https://schema.org/geoDisjoint>
pub const GEO_DISJOINT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/geoDisjoint";
/// <https://schema.org/geoDisjoint>
pub const GEO_DISJOINT_PROPERTY_LABEL: &str = "geoDisjoint";
pub struct GeoDisjointPropertyIri;
impl PartialEq<&str> for GeoDisjointPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_DISJOINT_PROPERTY_IRI_HTTP || *other == GEO_DISJOINT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GeoDisjointPropertyIri> for &str {
	fn eq(&self, other: &GeoDisjointPropertyIri) -> bool {
		*self == GEO_DISJOINT_PROPERTY_IRI_HTTP || *self == GEO_DISJOINT_PROPERTY_IRI_HTTPS
	}
}
pub struct GeoDisjointPropertyIriOrLabel;
impl PartialEq<&str> for GeoDisjointPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoDisjointPropertyIri || *other == GEO_DISJOINT_PROPERTY_LABEL
	}
}
impl PartialEq<GeoDisjointPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GeoDisjointPropertyIriOrLabel) -> bool {
		*self == GeoDisjointPropertyIri || *self == GEO_DISJOINT_PROPERTY_LABEL
	}
}
