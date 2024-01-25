/// <https://schema.org/SpokenWordAlbum>
pub const SPOKEN_WORD_ALBUM_IRI_HTTP: &str = "http://schema.org/SpokenWordAlbum";
/// <https://schema.org/SpokenWordAlbum>
pub const SPOKEN_WORD_ALBUM_IRI_HTTPS: &str = "https://schema.org/SpokenWordAlbum";
/// <https://schema.org/SpokenWordAlbum>
pub const SPOKEN_WORD_ALBUM_LABEL: &str = "SpokenWordAlbum";
pub struct SpokenWordAlbumIri;
impl PartialEq<&str> for SpokenWordAlbumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPOKEN_WORD_ALBUM_IRI_HTTP || *other == SPOKEN_WORD_ALBUM_IRI_HTTPS
	}
}
impl PartialEq<SpokenWordAlbumIri> for &str {
	fn eq(&self, other: &SpokenWordAlbumIri) -> bool {
		*self == SPOKEN_WORD_ALBUM_IRI_HTTP || *self == SPOKEN_WORD_ALBUM_IRI_HTTPS
	}
}
pub struct SpokenWordAlbumIriOrLabel;
impl PartialEq<&str> for SpokenWordAlbumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpokenWordAlbumIri || *other == SPOKEN_WORD_ALBUM_LABEL
	}
}
impl PartialEq<SpokenWordAlbumIriOrLabel> for &str {
	fn eq(&self, other: &SpokenWordAlbumIriOrLabel) -> bool {
		*self == SpokenWordAlbumIri || *self == SPOKEN_WORD_ALBUM_LABEL
	}
}
