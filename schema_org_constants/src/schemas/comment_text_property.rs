/// <https://schema.org/commentText>
pub const COMMENT_TEXT_PROPERTY_IRI_HTTP: &str = "http://schema.org/commentText";
/// <https://schema.org/commentText>
pub const COMMENT_TEXT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/commentText";
/// <https://schema.org/commentText>
pub const COMMENT_TEXT_PROPERTY_LABEL: &str = "commentText";
pub struct CommentTextPropertyIri;
impl PartialEq<&str> for CommentTextPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMMENT_TEXT_PROPERTY_IRI_HTTP || *other == COMMENT_TEXT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CommentTextPropertyIri> for &str {
	fn eq(&self, other: &CommentTextPropertyIri) -> bool {
		*self == COMMENT_TEXT_PROPERTY_IRI_HTTP || *self == COMMENT_TEXT_PROPERTY_IRI_HTTPS
	}
}
pub struct CommentTextPropertyIriOrLabel;
impl PartialEq<&str> for CommentTextPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CommentTextPropertyIri || *other == COMMENT_TEXT_PROPERTY_LABEL
	}
}
impl PartialEq<CommentTextPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CommentTextPropertyIriOrLabel) -> bool {
		*self == CommentTextPropertyIri || *self == COMMENT_TEXT_PROPERTY_LABEL
	}
}
