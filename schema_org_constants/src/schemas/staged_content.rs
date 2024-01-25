/// <https://schema.org/StagedContent>
pub const STAGED_CONTENT_IRI_HTTP: &str = "http://schema.org/StagedContent";
/// <https://schema.org/StagedContent>
pub const STAGED_CONTENT_IRI_HTTPS: &str = "https://schema.org/StagedContent";
/// <https://schema.org/StagedContent>
pub const STAGED_CONTENT_LABEL: &str = "StagedContent";
pub struct StagedContentIri;
impl PartialEq<&str> for StagedContentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STAGED_CONTENT_IRI_HTTP || *other == STAGED_CONTENT_IRI_HTTPS
	}
}
impl PartialEq<StagedContentIri> for &str {
	fn eq(&self, other: &StagedContentIri) -> bool {
		*self == STAGED_CONTENT_IRI_HTTP || *self == STAGED_CONTENT_IRI_HTTPS
	}
}
pub struct StagedContentIriOrLabel;
impl PartialEq<&str> for StagedContentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StagedContentIri || *other == STAGED_CONTENT_LABEL
	}
}
impl PartialEq<StagedContentIriOrLabel> for &str {
	fn eq(&self, other: &StagedContentIriOrLabel) -> bool {
		*self == StagedContentIri || *self == STAGED_CONTENT_LABEL
	}
}
