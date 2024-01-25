/// <https://schema.org/SoundtrackAlbum>
pub const SOUNDTRACK_ALBUM_IRI_HTTP: &str = "http://schema.org/SoundtrackAlbum";
/// <https://schema.org/SoundtrackAlbum>
pub const SOUNDTRACK_ALBUM_IRI_HTTPS: &str = "https://schema.org/SoundtrackAlbum";
/// <https://schema.org/SoundtrackAlbum>
pub const SOUNDTRACK_ALBUM_LABEL: &str = "SoundtrackAlbum";
pub struct SoundtrackAlbumIri;
impl PartialEq<&str> for SoundtrackAlbumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SOUNDTRACK_ALBUM_IRI_HTTP || *other == SOUNDTRACK_ALBUM_IRI_HTTPS
	}
}
impl PartialEq<SoundtrackAlbumIri> for &str {
	fn eq(&self, other: &SoundtrackAlbumIri) -> bool {
		*self == SOUNDTRACK_ALBUM_IRI_HTTP || *self == SOUNDTRACK_ALBUM_IRI_HTTPS
	}
}
pub struct SoundtrackAlbumIriOrLabel;
impl PartialEq<&str> for SoundtrackAlbumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SoundtrackAlbumIri || *other == SOUNDTRACK_ALBUM_LABEL
	}
}
impl PartialEq<SoundtrackAlbumIriOrLabel> for &str {
	fn eq(&self, other: &SoundtrackAlbumIriOrLabel) -> bool {
		*self == SoundtrackAlbumIri || *self == SOUNDTRACK_ALBUM_LABEL
	}
}
