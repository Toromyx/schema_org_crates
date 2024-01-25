/// <https://schema.org/geoMidpoint>
pub const GEO_MIDPOINT_PROPERTY_IRI_HTTP: &str = "http://schema.org/geoMidpoint";
/// <https://schema.org/geoMidpoint>
pub const GEO_MIDPOINT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/geoMidpoint";
/// <https://schema.org/geoMidpoint>
pub const GEO_MIDPOINT_PROPERTY_LABEL: &str = "geoMidpoint";
pub struct GeoMidpointPropertyIri;
impl PartialEq<&str> for GeoMidpointPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_MIDPOINT_PROPERTY_IRI_HTTP || *other == GEO_MIDPOINT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GeoMidpointPropertyIri> for &str {
	fn eq(&self, other: &GeoMidpointPropertyIri) -> bool {
		*self == GEO_MIDPOINT_PROPERTY_IRI_HTTP || *self == GEO_MIDPOINT_PROPERTY_IRI_HTTPS
	}
}
pub struct GeoMidpointPropertyIriOrLabel;
impl PartialEq<&str> for GeoMidpointPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoMidpointPropertyIri || *other == GEO_MIDPOINT_PROPERTY_LABEL
	}
}
impl PartialEq<GeoMidpointPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GeoMidpointPropertyIriOrLabel) -> bool {
		*self == GeoMidpointPropertyIri || *self == GEO_MIDPOINT_PROPERTY_LABEL
	}
}
