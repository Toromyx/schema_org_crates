/// <https://schema.org/DecontextualizedContent>
pub const DECONTEXTUALIZED_CONTENT_IRI_HTTP: &str = "http://schema.org/DecontextualizedContent";
/// <https://schema.org/DecontextualizedContent>
pub const DECONTEXTUALIZED_CONTENT_IRI_HTTPS: &str = "https://schema.org/DecontextualizedContent";
/// <https://schema.org/DecontextualizedContent>
pub const DECONTEXTUALIZED_CONTENT_LABEL: &str = "DecontextualizedContent";
pub struct DecontextualizedContentIri;
impl PartialEq<&str> for DecontextualizedContentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DECONTEXTUALIZED_CONTENT_IRI_HTTP || *other == DECONTEXTUALIZED_CONTENT_IRI_HTTPS
	}
}
impl PartialEq<DecontextualizedContentIri> for &str {
	fn eq(&self, other: &DecontextualizedContentIri) -> bool {
		*self == DECONTEXTUALIZED_CONTENT_IRI_HTTP || *self == DECONTEXTUALIZED_CONTENT_IRI_HTTPS
	}
}
pub struct DecontextualizedContentIriOrLabel;
impl PartialEq<&str> for DecontextualizedContentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DecontextualizedContentIri || *other == DECONTEXTUALIZED_CONTENT_LABEL
	}
}
impl PartialEq<DecontextualizedContentIriOrLabel> for &str {
	fn eq(&self, other: &DecontextualizedContentIriOrLabel) -> bool {
		*self == DecontextualizedContentIri || *self == DECONTEXTUALIZED_CONTENT_LABEL
	}
}
