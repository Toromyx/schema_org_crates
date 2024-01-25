/// <https://schema.org/TextObject>
pub const TEXT_OBJECT_IRI_HTTP: &str = "http://schema.org/TextObject";
/// <https://schema.org/TextObject>
pub const TEXT_OBJECT_IRI_HTTPS: &str = "https://schema.org/TextObject";
/// <https://schema.org/TextObject>
pub const TEXT_OBJECT_LABEL: &str = "TextObject";
pub struct TextObjectIri;
impl PartialEq<&str> for TextObjectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TEXT_OBJECT_IRI_HTTP || *other == TEXT_OBJECT_IRI_HTTPS
	}
}
impl PartialEq<TextObjectIri> for &str {
	fn eq(&self, other: &TextObjectIri) -> bool {
		*self == TEXT_OBJECT_IRI_HTTP || *self == TEXT_OBJECT_IRI_HTTPS
	}
}
pub struct TextObjectIriOrLabel;
impl PartialEq<&str> for TextObjectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TextObjectIri || *other == TEXT_OBJECT_LABEL
	}
}
impl PartialEq<TextObjectIriOrLabel> for &str {
	fn eq(&self, other: &TextObjectIriOrLabel) -> bool {
		*self == TextObjectIri || *self == TEXT_OBJECT_LABEL
	}
}
