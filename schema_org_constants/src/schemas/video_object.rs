/// <https://schema.org/VideoObject>
pub const VIDEO_OBJECT_IRI_HTTP: &str = "http://schema.org/VideoObject";
/// <https://schema.org/VideoObject>
pub const VIDEO_OBJECT_IRI_HTTPS: &str = "https://schema.org/VideoObject";
/// <https://schema.org/VideoObject>
pub const VIDEO_OBJECT_LABEL: &str = "VideoObject";
pub struct VideoObjectIri;
impl PartialEq<&str> for VideoObjectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIDEO_OBJECT_IRI_HTTP || *other == VIDEO_OBJECT_IRI_HTTPS
	}
}
impl PartialEq<VideoObjectIri> for &str {
	fn eq(&self, other: &VideoObjectIri) -> bool {
		*self == VIDEO_OBJECT_IRI_HTTP || *self == VIDEO_OBJECT_IRI_HTTPS
	}
}
pub struct VideoObjectIriOrLabel;
impl PartialEq<&str> for VideoObjectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VideoObjectIri || *other == VIDEO_OBJECT_LABEL
	}
}
impl PartialEq<VideoObjectIriOrLabel> for &str {
	fn eq(&self, other: &VideoObjectIriOrLabel) -> bool {
		*self == VideoObjectIri || *self == VIDEO_OBJECT_LABEL
	}
}
