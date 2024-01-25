/// <https://schema.org/text>
pub const TEXT_PROPERTY_IRI_HTTP: &str = "http://schema.org/text";
/// <https://schema.org/text>
pub const TEXT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/text";
/// <https://schema.org/text>
pub const TEXT_PROPERTY_LABEL: &str = "text";
pub struct TextPropertyIri;
impl PartialEq<&str> for TextPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TEXT_PROPERTY_IRI_HTTP || *other == TEXT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TextPropertyIri> for &str {
	fn eq(&self, other: &TextPropertyIri) -> bool {
		*self == TEXT_PROPERTY_IRI_HTTP || *self == TEXT_PROPERTY_IRI_HTTPS
	}
}
pub struct TextPropertyIriOrLabel;
impl PartialEq<&str> for TextPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TextPropertyIri || *other == TEXT_PROPERTY_LABEL
	}
}
impl PartialEq<TextPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TextPropertyIriOrLabel) -> bool {
		*self == TextPropertyIri || *self == TEXT_PROPERTY_LABEL
	}
}
