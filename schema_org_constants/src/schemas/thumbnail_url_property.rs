/// <https://schema.org/thumbnailUrl>
pub const THUMBNAIL_URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/thumbnailUrl";
/// <https://schema.org/thumbnailUrl>
pub const THUMBNAIL_URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/thumbnailUrl";
/// <https://schema.org/thumbnailUrl>
pub const THUMBNAIL_URL_PROPERTY_LABEL: &str = "thumbnailUrl";
pub struct ThumbnailUrlPropertyIri;
impl PartialEq<&str> for ThumbnailUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == THUMBNAIL_URL_PROPERTY_IRI_HTTP || *other == THUMBNAIL_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ThumbnailUrlPropertyIri> for &str {
	fn eq(&self, other: &ThumbnailUrlPropertyIri) -> bool {
		*self == THUMBNAIL_URL_PROPERTY_IRI_HTTP || *self == THUMBNAIL_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct ThumbnailUrlPropertyIriOrLabel;
impl PartialEq<&str> for ThumbnailUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ThumbnailUrlPropertyIri || *other == THUMBNAIL_URL_PROPERTY_LABEL
	}
}
impl PartialEq<ThumbnailUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ThumbnailUrlPropertyIriOrLabel) -> bool {
		*self == ThumbnailUrlPropertyIri || *self == THUMBNAIL_URL_PROPERTY_LABEL
	}
}
