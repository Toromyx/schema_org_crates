/// <https://schema.org/artworkSurface>
pub const ARTWORK_SURFACE_PROPERTY_IRI_HTTP: &str = "http://schema.org/artworkSurface";
/// <https://schema.org/artworkSurface>
pub const ARTWORK_SURFACE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/artworkSurface";
/// <https://schema.org/artworkSurface>
pub const ARTWORK_SURFACE_PROPERTY_LABEL: &str = "artworkSurface";
pub struct ArtworkSurfacePropertyIri;
impl PartialEq<&str> for ArtworkSurfacePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARTWORK_SURFACE_PROPERTY_IRI_HTTP || *other == ARTWORK_SURFACE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArtworkSurfacePropertyIri> for &str {
	fn eq(&self, other: &ArtworkSurfacePropertyIri) -> bool {
		*self == ARTWORK_SURFACE_PROPERTY_IRI_HTTP || *self == ARTWORK_SURFACE_PROPERTY_IRI_HTTPS
	}
}
pub struct ArtworkSurfacePropertyIriOrLabel;
impl PartialEq<&str> for ArtworkSurfacePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArtworkSurfacePropertyIri || *other == ARTWORK_SURFACE_PROPERTY_LABEL
	}
}
impl PartialEq<ArtworkSurfacePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArtworkSurfacePropertyIriOrLabel) -> bool {
		*self == ArtworkSurfacePropertyIri || *self == ARTWORK_SURFACE_PROPERTY_LABEL
	}
}
