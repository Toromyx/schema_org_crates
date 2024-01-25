/// <https://schema.org/textValue>
pub const TEXT_VALUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/textValue";
/// <https://schema.org/textValue>
pub const TEXT_VALUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/textValue";
/// <https://schema.org/textValue>
pub const TEXT_VALUE_PROPERTY_LABEL: &str = "textValue";
pub struct TextValuePropertyIri;
impl PartialEq<&str> for TextValuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TEXT_VALUE_PROPERTY_IRI_HTTP || *other == TEXT_VALUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TextValuePropertyIri> for &str {
	fn eq(&self, other: &TextValuePropertyIri) -> bool {
		*self == TEXT_VALUE_PROPERTY_IRI_HTTP || *self == TEXT_VALUE_PROPERTY_IRI_HTTPS
	}
}
pub struct TextValuePropertyIriOrLabel;
impl PartialEq<&str> for TextValuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TextValuePropertyIri || *other == TEXT_VALUE_PROPERTY_LABEL
	}
}
impl PartialEq<TextValuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TextValuePropertyIriOrLabel) -> bool {
		*self == TextValuePropertyIri || *self == TEXT_VALUE_PROPERTY_LABEL
	}
}
