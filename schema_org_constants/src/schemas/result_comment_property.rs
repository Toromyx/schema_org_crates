/// <https://schema.org/resultComment>
pub const RESULT_COMMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/resultComment";
/// <https://schema.org/resultComment>
pub const RESULT_COMMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/resultComment";
/// <https://schema.org/resultComment>
pub const RESULT_COMMENT_PROPERTY_LABEL: &str = "resultComment";
pub struct ResultCommentPropertyIri;
impl PartialEq<&str> for ResultCommentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESULT_COMMENT_PROPERTY_IRI_HTTP || *other == RESULT_COMMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ResultCommentPropertyIri> for &str {
	fn eq(&self, other: &ResultCommentPropertyIri) -> bool {
		*self == RESULT_COMMENT_PROPERTY_IRI_HTTP || *self == RESULT_COMMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct ResultCommentPropertyIriOrLabel;
impl PartialEq<&str> for ResultCommentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ResultCommentPropertyIri || *other == RESULT_COMMENT_PROPERTY_LABEL
	}
}
impl PartialEq<ResultCommentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ResultCommentPropertyIriOrLabel) -> bool {
		*self == ResultCommentPropertyIri || *self == RESULT_COMMENT_PROPERTY_LABEL
	}
}
