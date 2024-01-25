/// <https://schema.org/GeoShape>
pub const GEO_SHAPE_IRI_HTTP: &str = "http://schema.org/GeoShape";
/// <https://schema.org/GeoShape>
pub const GEO_SHAPE_IRI_HTTPS: &str = "https://schema.org/GeoShape";
/// <https://schema.org/GeoShape>
pub const GEO_SHAPE_LABEL: &str = "GeoShape";
pub struct GeoShapeIri;
impl PartialEq<&str> for GeoShapeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_SHAPE_IRI_HTTP || *other == GEO_SHAPE_IRI_HTTPS
	}
}
impl PartialEq<GeoShapeIri> for &str {
	fn eq(&self, other: &GeoShapeIri) -> bool {
		*self == GEO_SHAPE_IRI_HTTP || *self == GEO_SHAPE_IRI_HTTPS
	}
}
pub struct GeoShapeIriOrLabel;
impl PartialEq<&str> for GeoShapeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoShapeIri || *other == GEO_SHAPE_LABEL
	}
}
impl PartialEq<GeoShapeIriOrLabel> for &str {
	fn eq(&self, other: &GeoShapeIriOrLabel) -> bool {
		*self == GeoShapeIri || *self == GEO_SHAPE_LABEL
	}
}
