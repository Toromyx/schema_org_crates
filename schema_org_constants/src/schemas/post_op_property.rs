/// <https://schema.org/postOp>
pub const POST_OP_PROPERTY_IRI_HTTP: &str = "http://schema.org/postOp";
/// <https://schema.org/postOp>
pub const POST_OP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/postOp";
/// <https://schema.org/postOp>
pub const POST_OP_PROPERTY_LABEL: &str = "postOp";
pub struct PostOpPropertyIri;
impl PartialEq<&str> for PostOpPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POST_OP_PROPERTY_IRI_HTTP || *other == POST_OP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PostOpPropertyIri> for &str {
	fn eq(&self, other: &PostOpPropertyIri) -> bool {
		*self == POST_OP_PROPERTY_IRI_HTTP || *self == POST_OP_PROPERTY_IRI_HTTPS
	}
}
pub struct PostOpPropertyIriOrLabel;
impl PartialEq<&str> for PostOpPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PostOpPropertyIri || *other == POST_OP_PROPERTY_LABEL
	}
}
impl PartialEq<PostOpPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PostOpPropertyIriOrLabel) -> bool {
		*self == PostOpPropertyIri || *self == POST_OP_PROPERTY_LABEL
	}
}
