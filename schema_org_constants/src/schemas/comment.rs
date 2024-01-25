/// <https://schema.org/Comment>
pub const COMMENT_IRI_HTTP: &str = "http://schema.org/Comment";
/// <https://schema.org/Comment>
pub const COMMENT_IRI_HTTPS: &str = "https://schema.org/Comment";
/// <https://schema.org/Comment>
pub const COMMENT_LABEL: &str = "Comment";
pub struct CommentIri;
impl PartialEq<&str> for CommentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMMENT_IRI_HTTP || *other == COMMENT_IRI_HTTPS
	}
}
impl PartialEq<CommentIri> for &str {
	fn eq(&self, other: &CommentIri) -> bool {
		*self == COMMENT_IRI_HTTP || *self == COMMENT_IRI_HTTPS
	}
}
pub struct CommentIriOrLabel;
impl PartialEq<&str> for CommentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CommentIri || *other == COMMENT_LABEL
	}
}
impl PartialEq<CommentIriOrLabel> for &str {
	fn eq(&self, other: &CommentIriOrLabel) -> bool {
		*self == CommentIri || *self == COMMENT_LABEL
	}
}
