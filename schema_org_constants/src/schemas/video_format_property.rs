/// <https://schema.org/videoFormat>
pub const VIDEO_FORMAT_PROPERTY_IRI_HTTP: &str = "http://schema.org/videoFormat";
/// <https://schema.org/videoFormat>
pub const VIDEO_FORMAT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/videoFormat";
/// <https://schema.org/videoFormat>
pub const VIDEO_FORMAT_PROPERTY_LABEL: &str = "videoFormat";
pub struct VideoFormatPropertyIri;
impl PartialEq<&str> for VideoFormatPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIDEO_FORMAT_PROPERTY_IRI_HTTP || *other == VIDEO_FORMAT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VideoFormatPropertyIri> for &str {
	fn eq(&self, other: &VideoFormatPropertyIri) -> bool {
		*self == VIDEO_FORMAT_PROPERTY_IRI_HTTP || *self == VIDEO_FORMAT_PROPERTY_IRI_HTTPS
	}
}
pub struct VideoFormatPropertyIriOrLabel;
impl PartialEq<&str> for VideoFormatPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VideoFormatPropertyIri || *other == VIDEO_FORMAT_PROPERTY_LABEL
	}
}
impl PartialEq<VideoFormatPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VideoFormatPropertyIriOrLabel) -> bool {
		*self == VideoFormatPropertyIri || *self == VIDEO_FORMAT_PROPERTY_LABEL
	}
}
