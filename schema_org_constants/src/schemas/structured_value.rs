/// <https://schema.org/StructuredValue>
pub const STRUCTURED_VALUE_IRI_HTTP: &str = "http://schema.org/StructuredValue";
/// <https://schema.org/StructuredValue>
pub const STRUCTURED_VALUE_IRI_HTTPS: &str = "https://schema.org/StructuredValue";
/// <https://schema.org/StructuredValue>
pub const STRUCTURED_VALUE_LABEL: &str = "StructuredValue";
pub struct StructuredValueIri;
impl PartialEq<&str> for StructuredValueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STRUCTURED_VALUE_IRI_HTTP || *other == STRUCTURED_VALUE_IRI_HTTPS
	}
}
impl PartialEq<StructuredValueIri> for &str {
	fn eq(&self, other: &StructuredValueIri) -> bool {
		*self == STRUCTURED_VALUE_IRI_HTTP || *self == STRUCTURED_VALUE_IRI_HTTPS
	}
}
pub struct StructuredValueIriOrLabel;
impl PartialEq<&str> for StructuredValueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StructuredValueIri || *other == STRUCTURED_VALUE_LABEL
	}
}
impl PartialEq<StructuredValueIriOrLabel> for &str {
	fn eq(&self, other: &StructuredValueIriOrLabel) -> bool {
		*self == StructuredValueIri || *self == STRUCTURED_VALUE_LABEL
	}
}
