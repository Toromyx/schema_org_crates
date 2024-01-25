/// <https://schema.org/geo>
pub const GEO_PROPERTY_IRI_HTTP: &str = "http://schema.org/geo";
/// <https://schema.org/geo>
pub const GEO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/geo";
/// <https://schema.org/geo>
pub const GEO_PROPERTY_LABEL: &str = "geo";
pub struct GeoPropertyIri;
impl PartialEq<&str> for GeoPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_PROPERTY_IRI_HTTP || *other == GEO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GeoPropertyIri> for &str {
	fn eq(&self, other: &GeoPropertyIri) -> bool {
		*self == GEO_PROPERTY_IRI_HTTP || *self == GEO_PROPERTY_IRI_HTTPS
	}
}
pub struct GeoPropertyIriOrLabel;
impl PartialEq<&str> for GeoPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoPropertyIri || *other == GEO_PROPERTY_LABEL
	}
}
impl PartialEq<GeoPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GeoPropertyIriOrLabel) -> bool {
		*self == GeoPropertyIri || *self == GEO_PROPERTY_LABEL
	}
}
