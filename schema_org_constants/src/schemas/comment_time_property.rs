/// <https://schema.org/commentTime>
pub const COMMENT_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/commentTime";
/// <https://schema.org/commentTime>
pub const COMMENT_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/commentTime";
/// <https://schema.org/commentTime>
pub const COMMENT_TIME_PROPERTY_LABEL: &str = "commentTime";
pub struct CommentTimePropertyIri;
impl PartialEq<&str> for CommentTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMMENT_TIME_PROPERTY_IRI_HTTP || *other == COMMENT_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CommentTimePropertyIri> for &str {
	fn eq(&self, other: &CommentTimePropertyIri) -> bool {
		*self == COMMENT_TIME_PROPERTY_IRI_HTTP || *self == COMMENT_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct CommentTimePropertyIriOrLabel;
impl PartialEq<&str> for CommentTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CommentTimePropertyIri || *other == COMMENT_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<CommentTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CommentTimePropertyIriOrLabel) -> bool {
		*self == CommentTimePropertyIri || *self == COMMENT_TIME_PROPERTY_LABEL
	}
}
