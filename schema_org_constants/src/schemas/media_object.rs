/// <https://schema.org/MediaObject>
pub const MEDIA_OBJECT_IRI_HTTP: &str = "http://schema.org/MediaObject";
/// <https://schema.org/MediaObject>
pub const MEDIA_OBJECT_IRI_HTTPS: &str = "https://schema.org/MediaObject";
/// <https://schema.org/MediaObject>
pub const MEDIA_OBJECT_LABEL: &str = "MediaObject";
pub struct MediaObjectIri;
impl PartialEq<&str> for MediaObjectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDIA_OBJECT_IRI_HTTP || *other == MEDIA_OBJECT_IRI_HTTPS
	}
}
impl PartialEq<MediaObjectIri> for &str {
	fn eq(&self, other: &MediaObjectIri) -> bool {
		*self == MEDIA_OBJECT_IRI_HTTP || *self == MEDIA_OBJECT_IRI_HTTPS
	}
}
pub struct MediaObjectIriOrLabel;
impl PartialEq<&str> for MediaObjectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MediaObjectIri || *other == MEDIA_OBJECT_LABEL
	}
}
impl PartialEq<MediaObjectIriOrLabel> for &str {
	fn eq(&self, other: &MediaObjectIriOrLabel) -> bool {
		*self == MediaObjectIri || *self == MEDIA_OBJECT_LABEL
	}
}
