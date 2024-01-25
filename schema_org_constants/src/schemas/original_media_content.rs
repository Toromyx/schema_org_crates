/// <https://schema.org/OriginalMediaContent>
pub const ORIGINAL_MEDIA_CONTENT_IRI_HTTP: &str = "http://schema.org/OriginalMediaContent";
/// <https://schema.org/OriginalMediaContent>
pub const ORIGINAL_MEDIA_CONTENT_IRI_HTTPS: &str = "https://schema.org/OriginalMediaContent";
/// <https://schema.org/OriginalMediaContent>
pub const ORIGINAL_MEDIA_CONTENT_LABEL: &str = "OriginalMediaContent";
pub struct OriginalMediaContentIri;
impl PartialEq<&str> for OriginalMediaContentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORIGINAL_MEDIA_CONTENT_IRI_HTTP || *other == ORIGINAL_MEDIA_CONTENT_IRI_HTTPS
	}
}
impl PartialEq<OriginalMediaContentIri> for &str {
	fn eq(&self, other: &OriginalMediaContentIri) -> bool {
		*self == ORIGINAL_MEDIA_CONTENT_IRI_HTTP || *self == ORIGINAL_MEDIA_CONTENT_IRI_HTTPS
	}
}
pub struct OriginalMediaContentIriOrLabel;
impl PartialEq<&str> for OriginalMediaContentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OriginalMediaContentIri || *other == ORIGINAL_MEDIA_CONTENT_LABEL
	}
}
impl PartialEq<OriginalMediaContentIriOrLabel> for &str {
	fn eq(&self, other: &OriginalMediaContentIriOrLabel) -> bool {
		*self == OriginalMediaContentIri || *self == ORIGINAL_MEDIA_CONTENT_LABEL
	}
}
