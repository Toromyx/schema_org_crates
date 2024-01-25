/// <https://schema.org/GeoCoordinates>
pub const GEO_COORDINATES_IRI_HTTP: &str = "http://schema.org/GeoCoordinates";
/// <https://schema.org/GeoCoordinates>
pub const GEO_COORDINATES_IRI_HTTPS: &str = "https://schema.org/GeoCoordinates";
/// <https://schema.org/GeoCoordinates>
pub const GEO_COORDINATES_LABEL: &str = "GeoCoordinates";
pub struct GeoCoordinatesIri;
impl PartialEq<&str> for GeoCoordinatesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_COORDINATES_IRI_HTTP || *other == GEO_COORDINATES_IRI_HTTPS
	}
}
impl PartialEq<GeoCoordinatesIri> for &str {
	fn eq(&self, other: &GeoCoordinatesIri) -> bool {
		*self == GEO_COORDINATES_IRI_HTTP || *self == GEO_COORDINATES_IRI_HTTPS
	}
}
pub struct GeoCoordinatesIriOrLabel;
impl PartialEq<&str> for GeoCoordinatesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoCoordinatesIri || *other == GEO_COORDINATES_LABEL
	}
}
impl PartialEq<GeoCoordinatesIriOrLabel> for &str {
	fn eq(&self, other: &GeoCoordinatesIriOrLabel) -> bool {
		*self == GeoCoordinatesIri || *self == GEO_COORDINATES_LABEL
	}
}
