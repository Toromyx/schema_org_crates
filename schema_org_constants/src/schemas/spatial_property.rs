/// <https://schema.org/spatial>
pub const SPATIAL_PROPERTY_IRI_HTTP: &str = "http://schema.org/spatial";
/// <https://schema.org/spatial>
pub const SPATIAL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/spatial";
/// <https://schema.org/spatial>
pub const SPATIAL_PROPERTY_LABEL: &str = "spatial";
pub struct SpatialPropertyIri;
impl PartialEq<&str> for SpatialPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPATIAL_PROPERTY_IRI_HTTP || *other == SPATIAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SpatialPropertyIri> for &str {
	fn eq(&self, other: &SpatialPropertyIri) -> bool {
		*self == SPATIAL_PROPERTY_IRI_HTTP || *self == SPATIAL_PROPERTY_IRI_HTTPS
	}
}
pub struct SpatialPropertyIriOrLabel;
impl PartialEq<&str> for SpatialPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpatialPropertyIri || *other == SPATIAL_PROPERTY_LABEL
	}
}
impl PartialEq<SpatialPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SpatialPropertyIriOrLabel) -> bool {
		*self == SpatialPropertyIri || *self == SPATIAL_PROPERTY_LABEL
	}
}
