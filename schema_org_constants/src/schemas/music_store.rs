/// <https://schema.org/MusicStore>
pub const MUSIC_STORE_IRI_HTTP: &str = "http://schema.org/MusicStore";
/// <https://schema.org/MusicStore>
pub const MUSIC_STORE_IRI_HTTPS: &str = "https://schema.org/MusicStore";
/// <https://schema.org/MusicStore>
pub const MUSIC_STORE_LABEL: &str = "MusicStore";
pub struct MusicStoreIri;
impl PartialEq<&str> for MusicStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_STORE_IRI_HTTP || *other == MUSIC_STORE_IRI_HTTPS
	}
}
impl PartialEq<MusicStoreIri> for &str {
	fn eq(&self, other: &MusicStoreIri) -> bool {
		*self == MUSIC_STORE_IRI_HTTP || *self == MUSIC_STORE_IRI_HTTPS
	}
}
pub struct MusicStoreIriOrLabel;
impl PartialEq<&str> for MusicStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicStoreIri || *other == MUSIC_STORE_LABEL
	}
}
impl PartialEq<MusicStoreIriOrLabel> for &str {
	fn eq(&self, other: &MusicStoreIriOrLabel) -> bool {
		*self == MusicStoreIri || *self == MUSIC_STORE_LABEL
	}
}
