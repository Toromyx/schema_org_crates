/// <https://schema.org/videoQuality>
pub const VIDEO_QUALITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/videoQuality";
/// <https://schema.org/videoQuality>
pub const VIDEO_QUALITY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/videoQuality";
/// <https://schema.org/videoQuality>
pub const VIDEO_QUALITY_PROPERTY_LABEL: &str = "videoQuality";
pub struct VideoQualityPropertyIri;
impl PartialEq<&str> for VideoQualityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIDEO_QUALITY_PROPERTY_IRI_HTTP || *other == VIDEO_QUALITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VideoQualityPropertyIri> for &str {
	fn eq(&self, other: &VideoQualityPropertyIri) -> bool {
		*self == VIDEO_QUALITY_PROPERTY_IRI_HTTP || *self == VIDEO_QUALITY_PROPERTY_IRI_HTTPS
	}
}
pub struct VideoQualityPropertyIriOrLabel;
impl PartialEq<&str> for VideoQualityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VideoQualityPropertyIri || *other == VIDEO_QUALITY_PROPERTY_LABEL
	}
}
impl PartialEq<VideoQualityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VideoQualityPropertyIriOrLabel) -> bool {
		*self == VideoQualityPropertyIri || *self == VIDEO_QUALITY_PROPERTY_LABEL
	}
}
