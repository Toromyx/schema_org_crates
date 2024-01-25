/// <https://schema.org/videoFrameSize>
pub const VIDEO_FRAME_SIZE_PROPERTY_IRI_HTTP: &str = "http://schema.org/videoFrameSize";
/// <https://schema.org/videoFrameSize>
pub const VIDEO_FRAME_SIZE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/videoFrameSize";
/// <https://schema.org/videoFrameSize>
pub const VIDEO_FRAME_SIZE_PROPERTY_LABEL: &str = "videoFrameSize";
pub struct VideoFrameSizePropertyIri;
impl PartialEq<&str> for VideoFrameSizePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIDEO_FRAME_SIZE_PROPERTY_IRI_HTTP
			|| *other == VIDEO_FRAME_SIZE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VideoFrameSizePropertyIri> for &str {
	fn eq(&self, other: &VideoFrameSizePropertyIri) -> bool {
		*self == VIDEO_FRAME_SIZE_PROPERTY_IRI_HTTP || *self == VIDEO_FRAME_SIZE_PROPERTY_IRI_HTTPS
	}
}
pub struct VideoFrameSizePropertyIriOrLabel;
impl PartialEq<&str> for VideoFrameSizePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VideoFrameSizePropertyIri || *other == VIDEO_FRAME_SIZE_PROPERTY_LABEL
	}
}
impl PartialEq<VideoFrameSizePropertyIriOrLabel> for &str {
	fn eq(&self, other: &VideoFrameSizePropertyIriOrLabel) -> bool {
		*self == VideoFrameSizePropertyIri || *self == VIDEO_FRAME_SIZE_PROPERTY_LABEL
	}
}
