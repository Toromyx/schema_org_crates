/// <https://schema.org/geoWithin>
pub const GEO_WITHIN_PROPERTY_IRI_HTTP: &str = "http://schema.org/geoWithin";
/// <https://schema.org/geoWithin>
pub const GEO_WITHIN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/geoWithin";
/// <https://schema.org/geoWithin>
pub const GEO_WITHIN_PROPERTY_LABEL: &str = "geoWithin";
pub struct GeoWithinPropertyIri;
impl PartialEq<&str> for GeoWithinPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_WITHIN_PROPERTY_IRI_HTTP || *other == GEO_WITHIN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GeoWithinPropertyIri> for &str {
	fn eq(&self, other: &GeoWithinPropertyIri) -> bool {
		*self == GEO_WITHIN_PROPERTY_IRI_HTTP || *self == GEO_WITHIN_PROPERTY_IRI_HTTPS
	}
}
pub struct GeoWithinPropertyIriOrLabel;
impl PartialEq<&str> for GeoWithinPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoWithinPropertyIri || *other == GEO_WITHIN_PROPERTY_LABEL
	}
}
impl PartialEq<GeoWithinPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GeoWithinPropertyIriOrLabel) -> bool {
		*self == GeoWithinPropertyIri || *self == GEO_WITHIN_PROPERTY_LABEL
	}
}
