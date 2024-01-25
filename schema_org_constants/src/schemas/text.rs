/// <https://schema.org/Text>
pub const TEXT_IRI_HTTP: &str = "http://schema.org/Text";
/// <https://schema.org/Text>
pub const TEXT_IRI_HTTPS: &str = "https://schema.org/Text";
/// <https://schema.org/Text>
pub const TEXT_LABEL: &str = "Text";
pub struct TextIri;
impl PartialEq<&str> for TextIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TEXT_IRI_HTTP || *other == TEXT_IRI_HTTPS
	}
}
impl PartialEq<TextIri> for &str {
	fn eq(&self, other: &TextIri) -> bool {
		*self == TEXT_IRI_HTTP || *self == TEXT_IRI_HTTPS
	}
}
pub struct TextIriOrLabel;
impl PartialEq<&str> for TextIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TextIri || *other == TEXT_LABEL
	}
}
impl PartialEq<TextIriOrLabel> for &str {
	fn eq(&self, other: &TextIriOrLabel) -> bool {
		*self == TextIri || *self == TEXT_LABEL
	}
}
