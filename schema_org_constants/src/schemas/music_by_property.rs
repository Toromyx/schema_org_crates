/// <https://schema.org/musicBy>
pub const MUSIC_BY_PROPERTY_IRI_HTTP: &str = "http://schema.org/musicBy";
/// <https://schema.org/musicBy>
pub const MUSIC_BY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/musicBy";
/// <https://schema.org/musicBy>
pub const MUSIC_BY_PROPERTY_LABEL: &str = "musicBy";
pub struct MusicByPropertyIri;
impl PartialEq<&str> for MusicByPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_BY_PROPERTY_IRI_HTTP || *other == MUSIC_BY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MusicByPropertyIri> for &str {
	fn eq(&self, other: &MusicByPropertyIri) -> bool {
		*self == MUSIC_BY_PROPERTY_IRI_HTTP || *self == MUSIC_BY_PROPERTY_IRI_HTTPS
	}
}
pub struct MusicByPropertyIriOrLabel;
impl PartialEq<&str> for MusicByPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicByPropertyIri || *other == MUSIC_BY_PROPERTY_LABEL
	}
}
impl PartialEq<MusicByPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MusicByPropertyIriOrLabel) -> bool {
		*self == MusicByPropertyIri || *self == MUSIC_BY_PROPERTY_LABEL
	}
}
