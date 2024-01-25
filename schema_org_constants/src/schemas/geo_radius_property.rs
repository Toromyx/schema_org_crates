/// <https://schema.org/geoRadius>
pub const GEO_RADIUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/geoRadius";
/// <https://schema.org/geoRadius>
pub const GEO_RADIUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/geoRadius";
/// <https://schema.org/geoRadius>
pub const GEO_RADIUS_PROPERTY_LABEL: &str = "geoRadius";
pub struct GeoRadiusPropertyIri;
impl PartialEq<&str> for GeoRadiusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_RADIUS_PROPERTY_IRI_HTTP || *other == GEO_RADIUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GeoRadiusPropertyIri> for &str {
	fn eq(&self, other: &GeoRadiusPropertyIri) -> bool {
		*self == GEO_RADIUS_PROPERTY_IRI_HTTP || *self == GEO_RADIUS_PROPERTY_IRI_HTTPS
	}
}
pub struct GeoRadiusPropertyIriOrLabel;
impl PartialEq<&str> for GeoRadiusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoRadiusPropertyIri || *other == GEO_RADIUS_PROPERTY_LABEL
	}
}
impl PartialEq<GeoRadiusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GeoRadiusPropertyIriOrLabel) -> bool {
		*self == GeoRadiusPropertyIri || *self == GEO_RADIUS_PROPERTY_LABEL
	}
}
