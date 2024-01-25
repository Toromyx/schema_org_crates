/// <https://schema.org/surface>
#[deprecated = "This schema is superseded by <https://schema.org/artworkSurface>."]
pub const SURFACE_PROPERTY_IRI_HTTP: &str = "http://schema.org/surface";
/// <https://schema.org/surface>
#[deprecated = "This schema is superseded by <https://schema.org/artworkSurface>."]
pub const SURFACE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/surface";
/// <https://schema.org/surface>
#[deprecated = "This schema is superseded by <https://schema.org/artworkSurface>."]
pub const SURFACE_PROPERTY_LABEL: &str = "surface";
pub struct SurfacePropertyIri;
impl PartialEq<&str> for SurfacePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SURFACE_PROPERTY_IRI_HTTP || *other == SURFACE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SurfacePropertyIri> for &str {
	fn eq(&self, other: &SurfacePropertyIri) -> bool {
		*self == SURFACE_PROPERTY_IRI_HTTP || *self == SURFACE_PROPERTY_IRI_HTTPS
	}
}
pub struct SurfacePropertyIriOrLabel;
impl PartialEq<&str> for SurfacePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SurfacePropertyIri || *other == SURFACE_PROPERTY_LABEL
	}
}
impl PartialEq<SurfacePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SurfacePropertyIriOrLabel) -> bool {
		*self == SurfacePropertyIri || *self == SURFACE_PROPERTY_LABEL
	}
}
