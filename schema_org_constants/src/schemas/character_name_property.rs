/// <https://schema.org/characterName>
pub const CHARACTER_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/characterName";
/// <https://schema.org/characterName>
pub const CHARACTER_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/characterName";
/// <https://schema.org/characterName>
pub const CHARACTER_NAME_PROPERTY_LABEL: &str = "characterName";
pub struct CharacterNamePropertyIri;
impl PartialEq<&str> for CharacterNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHARACTER_NAME_PROPERTY_IRI_HTTP || *other == CHARACTER_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CharacterNamePropertyIri> for &str {
	fn eq(&self, other: &CharacterNamePropertyIri) -> bool {
		*self == CHARACTER_NAME_PROPERTY_IRI_HTTP || *self == CHARACTER_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct CharacterNamePropertyIriOrLabel;
impl PartialEq<&str> for CharacterNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CharacterNamePropertyIri || *other == CHARACTER_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<CharacterNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CharacterNamePropertyIriOrLabel) -> bool {
		*self == CharacterNamePropertyIri || *self == CHARACTER_NAME_PROPERTY_LABEL
	}
}
