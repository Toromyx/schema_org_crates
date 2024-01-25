/// <https://schema.org/character>
pub const CHARACTER_PROPERTY_IRI_HTTP: &str = "http://schema.org/character";
/// <https://schema.org/character>
pub const CHARACTER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/character";
/// <https://schema.org/character>
pub const CHARACTER_PROPERTY_LABEL: &str = "character";
pub struct CharacterPropertyIri;
impl PartialEq<&str> for CharacterPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHARACTER_PROPERTY_IRI_HTTP || *other == CHARACTER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CharacterPropertyIri> for &str {
	fn eq(&self, other: &CharacterPropertyIri) -> bool {
		*self == CHARACTER_PROPERTY_IRI_HTTP || *self == CHARACTER_PROPERTY_IRI_HTTPS
	}
}
pub struct CharacterPropertyIriOrLabel;
impl PartialEq<&str> for CharacterPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CharacterPropertyIri || *other == CHARACTER_PROPERTY_LABEL
	}
}
impl PartialEq<CharacterPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CharacterPropertyIriOrLabel) -> bool {
		*self == CharacterPropertyIri || *self == CHARACTER_PROPERTY_LABEL
	}
}
