/// <https://schema.org/comment>
pub const COMMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/comment";
/// <https://schema.org/comment>
pub const COMMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/comment";
/// <https://schema.org/comment>
pub const COMMENT_PROPERTY_LABEL: &str = "comment";
pub struct CommentPropertyIri;
impl PartialEq<&str> for CommentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMMENT_PROPERTY_IRI_HTTP || *other == COMMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CommentPropertyIri> for &str {
	fn eq(&self, other: &CommentPropertyIri) -> bool {
		*self == COMMENT_PROPERTY_IRI_HTTP || *self == COMMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct CommentPropertyIriOrLabel;
impl PartialEq<&str> for CommentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CommentPropertyIri || *other == COMMENT_PROPERTY_LABEL
	}
}
impl PartialEq<CommentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CommentPropertyIriOrLabel) -> bool {
		*self == CommentPropertyIri || *self == COMMENT_PROPERTY_LABEL
	}
}
