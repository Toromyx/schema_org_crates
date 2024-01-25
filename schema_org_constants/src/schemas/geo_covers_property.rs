/// <https://schema.org/geoCovers>
pub const GEO_COVERS_PROPERTY_IRI_HTTP: &str = "http://schema.org/geoCovers";
/// <https://schema.org/geoCovers>
pub const GEO_COVERS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/geoCovers";
/// <https://schema.org/geoCovers>
pub const GEO_COVERS_PROPERTY_LABEL: &str = "geoCovers";
pub struct GeoCoversPropertyIri;
impl PartialEq<&str> for GeoCoversPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_COVERS_PROPERTY_IRI_HTTP || *other == GEO_COVERS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GeoCoversPropertyIri> for &str {
	fn eq(&self, other: &GeoCoversPropertyIri) -> bool {
		*self == GEO_COVERS_PROPERTY_IRI_HTTP || *self == GEO_COVERS_PROPERTY_IRI_HTTPS
	}
}
pub struct GeoCoversPropertyIriOrLabel;
impl PartialEq<&str> for GeoCoversPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoCoversPropertyIri || *other == GEO_COVERS_PROPERTY_LABEL
	}
}
impl PartialEq<GeoCoversPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GeoCoversPropertyIriOrLabel) -> bool {
		*self == GeoCoversPropertyIri || *self == GEO_COVERS_PROPERTY_LABEL
	}
}
