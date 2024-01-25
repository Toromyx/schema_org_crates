/// <https://schema.org/MusicAlbum>
pub const MUSIC_ALBUM_IRI_HTTP: &str = "http://schema.org/MusicAlbum";
/// <https://schema.org/MusicAlbum>
pub const MUSIC_ALBUM_IRI_HTTPS: &str = "https://schema.org/MusicAlbum";
/// <https://schema.org/MusicAlbum>
pub const MUSIC_ALBUM_LABEL: &str = "MusicAlbum";
pub struct MusicAlbumIri;
impl PartialEq<&str> for MusicAlbumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_ALBUM_IRI_HTTP || *other == MUSIC_ALBUM_IRI_HTTPS
	}
}
impl PartialEq<MusicAlbumIri> for &str {
	fn eq(&self, other: &MusicAlbumIri) -> bool {
		*self == MUSIC_ALBUM_IRI_HTTP || *self == MUSIC_ALBUM_IRI_HTTPS
	}
}
pub struct MusicAlbumIriOrLabel;
impl PartialEq<&str> for MusicAlbumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicAlbumIri || *other == MUSIC_ALBUM_LABEL
	}
}
impl PartialEq<MusicAlbumIriOrLabel> for &str {
	fn eq(&self, other: &MusicAlbumIriOrLabel) -> bool {
		*self == MusicAlbumIri || *self == MUSIC_ALBUM_LABEL
	}
}
