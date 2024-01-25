/// <https://schema.org/thumbnail>
pub const THUMBNAIL_PROPERTY_IRI_HTTP: &str = "http://schema.org/thumbnail";
/// <https://schema.org/thumbnail>
pub const THUMBNAIL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/thumbnail";
/// <https://schema.org/thumbnail>
pub const THUMBNAIL_PROPERTY_LABEL: &str = "thumbnail";
pub struct ThumbnailPropertyIri;
impl PartialEq<&str> for ThumbnailPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == THUMBNAIL_PROPERTY_IRI_HTTP || *other == THUMBNAIL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ThumbnailPropertyIri> for &str {
	fn eq(&self, other: &ThumbnailPropertyIri) -> bool {
		*self == THUMBNAIL_PROPERTY_IRI_HTTP || *self == THUMBNAIL_PROPERTY_IRI_HTTPS
	}
}
pub struct ThumbnailPropertyIriOrLabel;
impl PartialEq<&str> for ThumbnailPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ThumbnailPropertyIri || *other == THUMBNAIL_PROPERTY_LABEL
	}
}
impl PartialEq<ThumbnailPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ThumbnailPropertyIriOrLabel) -> bool {
		*self == ThumbnailPropertyIri || *self == THUMBNAIL_PROPERTY_LABEL
	}
}
