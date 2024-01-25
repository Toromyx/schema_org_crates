/// <https://schema.org/musicalKey>
pub const MUSICAL_KEY_PROPERTY_IRI_HTTP: &str = "http://schema.org/musicalKey";
/// <https://schema.org/musicalKey>
pub const MUSICAL_KEY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/musicalKey";
/// <https://schema.org/musicalKey>
pub const MUSICAL_KEY_PROPERTY_LABEL: &str = "musicalKey";
pub struct MusicalKeyPropertyIri;
impl PartialEq<&str> for MusicalKeyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSICAL_KEY_PROPERTY_IRI_HTTP || *other == MUSICAL_KEY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MusicalKeyPropertyIri> for &str {
	fn eq(&self, other: &MusicalKeyPropertyIri) -> bool {
		*self == MUSICAL_KEY_PROPERTY_IRI_HTTP || *self == MUSICAL_KEY_PROPERTY_IRI_HTTPS
	}
}
pub struct MusicalKeyPropertyIriOrLabel;
impl PartialEq<&str> for MusicalKeyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicalKeyPropertyIri || *other == MUSICAL_KEY_PROPERTY_LABEL
	}
}
impl PartialEq<MusicalKeyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MusicalKeyPropertyIriOrLabel) -> bool {
		*self == MusicalKeyPropertyIri || *self == MUSICAL_KEY_PROPERTY_LABEL
	}
}
