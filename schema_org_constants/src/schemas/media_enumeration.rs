/// <https://schema.org/MediaEnumeration>
pub const MEDIA_ENUMERATION_IRI_HTTP: &str = "http://schema.org/MediaEnumeration";
/// <https://schema.org/MediaEnumeration>
pub const MEDIA_ENUMERATION_IRI_HTTPS: &str = "https://schema.org/MediaEnumeration";
/// <https://schema.org/MediaEnumeration>
pub const MEDIA_ENUMERATION_LABEL: &str = "MediaEnumeration";
pub struct MediaEnumerationIri;
impl PartialEq<&str> for MediaEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDIA_ENUMERATION_IRI_HTTP || *other == MEDIA_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<MediaEnumerationIri> for &str {
	fn eq(&self, other: &MediaEnumerationIri) -> bool {
		*self == MEDIA_ENUMERATION_IRI_HTTP || *self == MEDIA_ENUMERATION_IRI_HTTPS
	}
}
pub struct MediaEnumerationIriOrLabel;
impl PartialEq<&str> for MediaEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MediaEnumerationIri || *other == MEDIA_ENUMERATION_LABEL
	}
}
impl PartialEq<MediaEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &MediaEnumerationIriOrLabel) -> bool {
		*self == MediaEnumerationIri || *self == MEDIA_ENUMERATION_LABEL
	}
}
