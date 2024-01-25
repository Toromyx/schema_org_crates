/// <https://schema.org/CommentAction>
pub const COMMENT_ACTION_IRI_HTTP: &str = "http://schema.org/CommentAction";
/// <https://schema.org/CommentAction>
pub const COMMENT_ACTION_IRI_HTTPS: &str = "https://schema.org/CommentAction";
/// <https://schema.org/CommentAction>
pub const COMMENT_ACTION_LABEL: &str = "CommentAction";
pub struct CommentActionIri;
impl PartialEq<&str> for CommentActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMMENT_ACTION_IRI_HTTP || *other == COMMENT_ACTION_IRI_HTTPS
	}
}
impl PartialEq<CommentActionIri> for &str {
	fn eq(&self, other: &CommentActionIri) -> bool {
		*self == COMMENT_ACTION_IRI_HTTP || *self == COMMENT_ACTION_IRI_HTTPS
	}
}
pub struct CommentActionIriOrLabel;
impl PartialEq<&str> for CommentActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CommentActionIri || *other == COMMENT_ACTION_LABEL
	}
}
impl PartialEq<CommentActionIriOrLabel> for &str {
	fn eq(&self, other: &CommentActionIriOrLabel) -> bool {
		*self == CommentActionIri || *self == COMMENT_ACTION_LABEL
	}
}
