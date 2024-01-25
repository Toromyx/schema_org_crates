/// <https://schema.org/StudioAlbum>
pub const STUDIO_ALBUM_IRI_HTTP: &str = "http://schema.org/StudioAlbum";
/// <https://schema.org/StudioAlbum>
pub const STUDIO_ALBUM_IRI_HTTPS: &str = "https://schema.org/StudioAlbum";
/// <https://schema.org/StudioAlbum>
pub const STUDIO_ALBUM_LABEL: &str = "StudioAlbum";
pub struct StudioAlbumIri;
impl PartialEq<&str> for StudioAlbumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STUDIO_ALBUM_IRI_HTTP || *other == STUDIO_ALBUM_IRI_HTTPS
	}
}
impl PartialEq<StudioAlbumIri> for &str {
	fn eq(&self, other: &StudioAlbumIri) -> bool {
		*self == STUDIO_ALBUM_IRI_HTTP || *self == STUDIO_ALBUM_IRI_HTTPS
	}
}
pub struct StudioAlbumIriOrLabel;
impl PartialEq<&str> for StudioAlbumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StudioAlbumIri || *other == STUDIO_ALBUM_LABEL
	}
}
impl PartialEq<StudioAlbumIriOrLabel> for &str {
	fn eq(&self, other: &StudioAlbumIriOrLabel) -> bool {
		*self == StudioAlbumIri || *self == STUDIO_ALBUM_LABEL
	}
}
