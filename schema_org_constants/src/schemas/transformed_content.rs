/// <https://schema.org/TransformedContent>
pub const TRANSFORMED_CONTENT_IRI_HTTP: &str = "http://schema.org/TransformedContent";
/// <https://schema.org/TransformedContent>
pub const TRANSFORMED_CONTENT_IRI_HTTPS: &str = "https://schema.org/TransformedContent";
/// <https://schema.org/TransformedContent>
pub const TRANSFORMED_CONTENT_LABEL: &str = "TransformedContent";
pub struct TransformedContentIri;
impl PartialEq<&str> for TransformedContentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRANSFORMED_CONTENT_IRI_HTTP || *other == TRANSFORMED_CONTENT_IRI_HTTPS
	}
}
impl PartialEq<TransformedContentIri> for &str {
	fn eq(&self, other: &TransformedContentIri) -> bool {
		*self == TRANSFORMED_CONTENT_IRI_HTTP || *self == TRANSFORMED_CONTENT_IRI_HTTPS
	}
}
pub struct TransformedContentIriOrLabel;
impl PartialEq<&str> for TransformedContentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TransformedContentIri || *other == TRANSFORMED_CONTENT_LABEL
	}
}
impl PartialEq<TransformedContentIriOrLabel> for &str {
	fn eq(&self, other: &TransformedContentIriOrLabel) -> bool {
		*self == TransformedContentIri || *self == TRANSFORMED_CONTENT_LABEL
	}
}
