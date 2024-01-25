/// <https://schema.org/characterAttribute>
pub const CHARACTER_ATTRIBUTE_PROPERTY_IRI_HTTP: &str = "http://schema.org/characterAttribute";
/// <https://schema.org/characterAttribute>
pub const CHARACTER_ATTRIBUTE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/characterAttribute";
/// <https://schema.org/characterAttribute>
pub const CHARACTER_ATTRIBUTE_PROPERTY_LABEL: &str = "characterAttribute";
pub struct CharacterAttributePropertyIri;
impl PartialEq<&str> for CharacterAttributePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHARACTER_ATTRIBUTE_PROPERTY_IRI_HTTP
			|| *other == CHARACTER_ATTRIBUTE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CharacterAttributePropertyIri> for &str {
	fn eq(&self, other: &CharacterAttributePropertyIri) -> bool {
		*self == CHARACTER_ATTRIBUTE_PROPERTY_IRI_HTTP
			|| *self == CHARACTER_ATTRIBUTE_PROPERTY_IRI_HTTPS
	}
}
pub struct CharacterAttributePropertyIriOrLabel;
impl PartialEq<&str> for CharacterAttributePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CharacterAttributePropertyIri || *other == CHARACTER_ATTRIBUTE_PROPERTY_LABEL
	}
}
impl PartialEq<CharacterAttributePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CharacterAttributePropertyIriOrLabel) -> bool {
		*self == CharacterAttributePropertyIri || *self == CHARACTER_ATTRIBUTE_PROPERTY_LABEL
	}
}
