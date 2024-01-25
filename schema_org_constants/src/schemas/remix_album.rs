/// <https://schema.org/RemixAlbum>
pub const REMIX_ALBUM_IRI_HTTP: &str = "http://schema.org/RemixAlbum";
/// <https://schema.org/RemixAlbum>
pub const REMIX_ALBUM_IRI_HTTPS: &str = "https://schema.org/RemixAlbum";
/// <https://schema.org/RemixAlbum>
pub const REMIX_ALBUM_LABEL: &str = "RemixAlbum";
pub struct RemixAlbumIri;
impl PartialEq<&str> for RemixAlbumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REMIX_ALBUM_IRI_HTTP || *other == REMIX_ALBUM_IRI_HTTPS
	}
}
impl PartialEq<RemixAlbumIri> for &str {
	fn eq(&self, other: &RemixAlbumIri) -> bool {
		*self == REMIX_ALBUM_IRI_HTTP || *self == REMIX_ALBUM_IRI_HTTPS
	}
}
pub struct RemixAlbumIriOrLabel;
impl PartialEq<&str> for RemixAlbumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RemixAlbumIri || *other == REMIX_ALBUM_LABEL
	}
}
impl PartialEq<RemixAlbumIriOrLabel> for &str {
	fn eq(&self, other: &RemixAlbumIriOrLabel) -> bool {
		*self == RemixAlbumIri || *self == REMIX_ALBUM_LABEL
	}
}
