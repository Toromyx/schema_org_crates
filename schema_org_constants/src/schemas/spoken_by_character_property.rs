/// <https://schema.org/spokenByCharacter>
pub const SPOKEN_BY_CHARACTER_PROPERTY_IRI_HTTP: &str = "http://schema.org/spokenByCharacter";
/// <https://schema.org/spokenByCharacter>
pub const SPOKEN_BY_CHARACTER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/spokenByCharacter";
/// <https://schema.org/spokenByCharacter>
pub const SPOKEN_BY_CHARACTER_PROPERTY_LABEL: &str = "spokenByCharacter";
pub struct SpokenByCharacterPropertyIri;
impl PartialEq<&str> for SpokenByCharacterPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPOKEN_BY_CHARACTER_PROPERTY_IRI_HTTP
			|| *other == SPOKEN_BY_CHARACTER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SpokenByCharacterPropertyIri> for &str {
	fn eq(&self, other: &SpokenByCharacterPropertyIri) -> bool {
		*self == SPOKEN_BY_CHARACTER_PROPERTY_IRI_HTTP
			|| *self == SPOKEN_BY_CHARACTER_PROPERTY_IRI_HTTPS
	}
}
pub struct SpokenByCharacterPropertyIriOrLabel;
impl PartialEq<&str> for SpokenByCharacterPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpokenByCharacterPropertyIri || *other == SPOKEN_BY_CHARACTER_PROPERTY_LABEL
	}
}
impl PartialEq<SpokenByCharacterPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SpokenByCharacterPropertyIriOrLabel) -> bool {
		*self == SpokenByCharacterPropertyIri || *self == SPOKEN_BY_CHARACTER_PROPERTY_LABEL
	}
}
