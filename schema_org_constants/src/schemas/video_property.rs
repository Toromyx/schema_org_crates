/// <https://schema.org/video>
pub const VIDEO_PROPERTY_IRI_HTTP: &str = "http://schema.org/video";
/// <https://schema.org/video>
pub const VIDEO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/video";
/// <https://schema.org/video>
pub const VIDEO_PROPERTY_LABEL: &str = "video";
pub struct VideoPropertyIri;
impl PartialEq<&str> for VideoPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIDEO_PROPERTY_IRI_HTTP || *other == VIDEO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VideoPropertyIri> for &str {
	fn eq(&self, other: &VideoPropertyIri) -> bool {
		*self == VIDEO_PROPERTY_IRI_HTTP || *self == VIDEO_PROPERTY_IRI_HTTPS
	}
}
pub struct VideoPropertyIriOrLabel;
impl PartialEq<&str> for VideoPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VideoPropertyIri || *other == VIDEO_PROPERTY_LABEL
	}
}
impl PartialEq<VideoPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VideoPropertyIriOrLabel) -> bool {
		*self == VideoPropertyIri || *self == VIDEO_PROPERTY_LABEL
	}
}
