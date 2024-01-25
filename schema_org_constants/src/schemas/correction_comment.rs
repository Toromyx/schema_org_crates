/// <https://schema.org/CorrectionComment>
pub const CORRECTION_COMMENT_IRI_HTTP: &str = "http://schema.org/CorrectionComment";
/// <https://schema.org/CorrectionComment>
pub const CORRECTION_COMMENT_IRI_HTTPS: &str = "https://schema.org/CorrectionComment";
/// <https://schema.org/CorrectionComment>
pub const CORRECTION_COMMENT_LABEL: &str = "CorrectionComment";
pub struct CorrectionCommentIri;
impl PartialEq<&str> for CorrectionCommentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CORRECTION_COMMENT_IRI_HTTP || *other == CORRECTION_COMMENT_IRI_HTTPS
	}
}
impl PartialEq<CorrectionCommentIri> for &str {
	fn eq(&self, other: &CorrectionCommentIri) -> bool {
		*self == CORRECTION_COMMENT_IRI_HTTP || *self == CORRECTION_COMMENT_IRI_HTTPS
	}
}
pub struct CorrectionCommentIriOrLabel;
impl PartialEq<&str> for CorrectionCommentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CorrectionCommentIri || *other == CORRECTION_COMMENT_LABEL
	}
}
impl PartialEq<CorrectionCommentIriOrLabel> for &str {
	fn eq(&self, other: &CorrectionCommentIriOrLabel) -> bool {
		*self == CorrectionCommentIri || *self == CORRECTION_COMMENT_LABEL
	}
}
