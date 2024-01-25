/// <https://schema.org/MusicEvent>
pub const MUSIC_EVENT_IRI_HTTP: &str = "http://schema.org/MusicEvent";
/// <https://schema.org/MusicEvent>
pub const MUSIC_EVENT_IRI_HTTPS: &str = "https://schema.org/MusicEvent";
/// <https://schema.org/MusicEvent>
pub const MUSIC_EVENT_LABEL: &str = "MusicEvent";
pub struct MusicEventIri;
impl PartialEq<&str> for MusicEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_EVENT_IRI_HTTP || *other == MUSIC_EVENT_IRI_HTTPS
	}
}
impl PartialEq<MusicEventIri> for &str {
	fn eq(&self, other: &MusicEventIri) -> bool {
		*self == MUSIC_EVENT_IRI_HTTP || *self == MUSIC_EVENT_IRI_HTTPS
	}
}
pub struct MusicEventIriOrLabel;
impl PartialEq<&str> for MusicEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicEventIri || *other == MUSIC_EVENT_LABEL
	}
}
impl PartialEq<MusicEventIriOrLabel> for &str {
	fn eq(&self, other: &MusicEventIriOrLabel) -> bool {
		*self == MusicEventIri || *self == MUSIC_EVENT_LABEL
	}
}
