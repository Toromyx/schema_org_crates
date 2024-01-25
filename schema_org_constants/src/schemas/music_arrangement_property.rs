/// <https://schema.org/musicArrangement>
pub const MUSIC_ARRANGEMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/musicArrangement";
/// <https://schema.org/musicArrangement>
pub const MUSIC_ARRANGEMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/musicArrangement";
/// <https://schema.org/musicArrangement>
pub const MUSIC_ARRANGEMENT_PROPERTY_LABEL: &str = "musicArrangement";
pub struct MusicArrangementPropertyIri;
impl PartialEq<&str> for MusicArrangementPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_ARRANGEMENT_PROPERTY_IRI_HTTP
			|| *other == MUSIC_ARRANGEMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MusicArrangementPropertyIri> for &str {
	fn eq(&self, other: &MusicArrangementPropertyIri) -> bool {
		*self == MUSIC_ARRANGEMENT_PROPERTY_IRI_HTTP
			|| *self == MUSIC_ARRANGEMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct MusicArrangementPropertyIriOrLabel;
impl PartialEq<&str> for MusicArrangementPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicArrangementPropertyIri || *other == MUSIC_ARRANGEMENT_PROPERTY_LABEL
	}
}
impl PartialEq<MusicArrangementPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MusicArrangementPropertyIriOrLabel) -> bool {
		*self == MusicArrangementPropertyIri || *self == MUSIC_ARRANGEMENT_PROPERTY_LABEL
	}
}
