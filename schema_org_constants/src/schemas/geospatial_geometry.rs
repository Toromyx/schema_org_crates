/// <https://schema.org/GeospatialGeometry>
pub const GEOSPATIAL_GEOMETRY_IRI_HTTP: &str = "http://schema.org/GeospatialGeometry";
/// <https://schema.org/GeospatialGeometry>
pub const GEOSPATIAL_GEOMETRY_IRI_HTTPS: &str = "https://schema.org/GeospatialGeometry";
/// <https://schema.org/GeospatialGeometry>
pub const GEOSPATIAL_GEOMETRY_LABEL: &str = "GeospatialGeometry";
pub struct GeospatialGeometryIri;
impl PartialEq<&str> for GeospatialGeometryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEOSPATIAL_GEOMETRY_IRI_HTTP || *other == GEOSPATIAL_GEOMETRY_IRI_HTTPS
	}
}
impl PartialEq<GeospatialGeometryIri> for &str {
	fn eq(&self, other: &GeospatialGeometryIri) -> bool {
		*self == GEOSPATIAL_GEOMETRY_IRI_HTTP || *self == GEOSPATIAL_GEOMETRY_IRI_HTTPS
	}
}
pub struct GeospatialGeometryIriOrLabel;
impl PartialEq<&str> for GeospatialGeometryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeospatialGeometryIri || *other == GEOSPATIAL_GEOMETRY_LABEL
	}
}
impl PartialEq<GeospatialGeometryIriOrLabel> for &str {
	fn eq(&self, other: &GeospatialGeometryIriOrLabel) -> bool {
		*self == GeospatialGeometryIri || *self == GEOSPATIAL_GEOMETRY_LABEL
	}
}
