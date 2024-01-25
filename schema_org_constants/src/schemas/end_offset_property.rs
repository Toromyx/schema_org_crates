/// <https://schema.org/endOffset>
pub const END_OFFSET_PROPERTY_IRI_HTTP: &str = "http://schema.org/endOffset";
/// <https://schema.org/endOffset>
pub const END_OFFSET_PROPERTY_IRI_HTTPS: &str = "https://schema.org/endOffset";
/// <https://schema.org/endOffset>
pub const END_OFFSET_PROPERTY_LABEL: &str = "endOffset";
pub struct EndOffsetPropertyIri;
impl PartialEq<&str> for EndOffsetPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == END_OFFSET_PROPERTY_IRI_HTTP || *other == END_OFFSET_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EndOffsetPropertyIri> for &str {
	fn eq(&self, other: &EndOffsetPropertyIri) -> bool {
		*self == END_OFFSET_PROPERTY_IRI_HTTP || *self == END_OFFSET_PROPERTY_IRI_HTTPS
	}
}
pub struct EndOffsetPropertyIriOrLabel;
impl PartialEq<&str> for EndOffsetPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EndOffsetPropertyIri || *other == END_OFFSET_PROPERTY_LABEL
	}
}
impl PartialEq<EndOffsetPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EndOffsetPropertyIriOrLabel) -> bool {
		*self == EndOffsetPropertyIri || *self == END_OFFSET_PROPERTY_LABEL
	}
}
