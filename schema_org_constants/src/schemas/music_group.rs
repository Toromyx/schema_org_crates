/// <https://schema.org/MusicGroup>
pub const MUSIC_GROUP_IRI_HTTP: &str = "http://schema.org/MusicGroup";
/// <https://schema.org/MusicGroup>
pub const MUSIC_GROUP_IRI_HTTPS: &str = "https://schema.org/MusicGroup";
/// <https://schema.org/MusicGroup>
pub const MUSIC_GROUP_LABEL: &str = "MusicGroup";
pub struct MusicGroupIri;
impl PartialEq<&str> for MusicGroupIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_GROUP_IRI_HTTP || *other == MUSIC_GROUP_IRI_HTTPS
	}
}
impl PartialEq<MusicGroupIri> for &str {
	fn eq(&self, other: &MusicGroupIri) -> bool {
		*self == MUSIC_GROUP_IRI_HTTP || *self == MUSIC_GROUP_IRI_HTTPS
	}
}
pub struct MusicGroupIriOrLabel;
impl PartialEq<&str> for MusicGroupIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicGroupIri || *other == MUSIC_GROUP_LABEL
	}
}
impl PartialEq<MusicGroupIriOrLabel> for &str {
	fn eq(&self, other: &MusicGroupIriOrLabel) -> bool {
		*self == MusicGroupIri || *self == MUSIC_GROUP_LABEL
	}
}
