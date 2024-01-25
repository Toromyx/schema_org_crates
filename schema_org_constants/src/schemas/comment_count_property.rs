/// <https://schema.org/commentCount>
pub const COMMENT_COUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/commentCount";
/// <https://schema.org/commentCount>
pub const COMMENT_COUNT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/commentCount";
/// <https://schema.org/commentCount>
pub const COMMENT_COUNT_PROPERTY_LABEL: &str = "commentCount";
pub struct CommentCountPropertyIri;
impl PartialEq<&str> for CommentCountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMMENT_COUNT_PROPERTY_IRI_HTTP || *other == COMMENT_COUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CommentCountPropertyIri> for &str {
	fn eq(&self, other: &CommentCountPropertyIri) -> bool {
		*self == COMMENT_COUNT_PROPERTY_IRI_HTTP || *self == COMMENT_COUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct CommentCountPropertyIriOrLabel;
impl PartialEq<&str> for CommentCountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CommentCountPropertyIri || *other == COMMENT_COUNT_PROPERTY_LABEL
	}
}
impl PartialEq<CommentCountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CommentCountPropertyIriOrLabel) -> bool {
		*self == CommentCountPropertyIri || *self == COMMENT_COUNT_PROPERTY_LABEL
	}
}
