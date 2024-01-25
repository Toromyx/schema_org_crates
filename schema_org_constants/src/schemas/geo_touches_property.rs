/// <https://schema.org/geoTouches>
pub const GEO_TOUCHES_PROPERTY_IRI_HTTP: &str = "http://schema.org/geoTouches";
/// <https://schema.org/geoTouches>
pub const GEO_TOUCHES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/geoTouches";
/// <https://schema.org/geoTouches>
pub const GEO_TOUCHES_PROPERTY_LABEL: &str = "geoTouches";
pub struct GeoTouchesPropertyIri;
impl PartialEq<&str> for GeoTouchesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_TOUCHES_PROPERTY_IRI_HTTP || *other == GEO_TOUCHES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GeoTouchesPropertyIri> for &str {
	fn eq(&self, other: &GeoTouchesPropertyIri) -> bool {
		*self == GEO_TOUCHES_PROPERTY_IRI_HTTP || *self == GEO_TOUCHES_PROPERTY_IRI_HTTPS
	}
}
pub struct GeoTouchesPropertyIriOrLabel;
impl PartialEq<&str> for GeoTouchesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoTouchesPropertyIri || *other == GEO_TOUCHES_PROPERTY_LABEL
	}
}
impl PartialEq<GeoTouchesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GeoTouchesPropertyIriOrLabel) -> bool {
		*self == GeoTouchesPropertyIri || *self == GEO_TOUCHES_PROPERTY_LABEL
	}
}
