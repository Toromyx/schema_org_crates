/// <https://schema.org/geoContains>
pub const GEO_CONTAINS_PROPERTY_IRI_HTTP: &str = "http://schema.org/geoContains";
/// <https://schema.org/geoContains>
pub const GEO_CONTAINS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/geoContains";
/// <https://schema.org/geoContains>
pub const GEO_CONTAINS_PROPERTY_LABEL: &str = "geoContains";
pub struct GeoContainsPropertyIri;
impl PartialEq<&str> for GeoContainsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_CONTAINS_PROPERTY_IRI_HTTP || *other == GEO_CONTAINS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GeoContainsPropertyIri> for &str {
	fn eq(&self, other: &GeoContainsPropertyIri) -> bool {
		*self == GEO_CONTAINS_PROPERTY_IRI_HTTP || *self == GEO_CONTAINS_PROPERTY_IRI_HTTPS
	}
}
pub struct GeoContainsPropertyIriOrLabel;
impl PartialEq<&str> for GeoContainsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoContainsPropertyIri || *other == GEO_CONTAINS_PROPERTY_LABEL
	}
}
impl PartialEq<GeoContainsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GeoContainsPropertyIriOrLabel) -> bool {
		*self == GeoContainsPropertyIri || *self == GEO_CONTAINS_PROPERTY_LABEL
	}
}
