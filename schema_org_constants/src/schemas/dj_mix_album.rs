/// <https://schema.org/DJMixAlbum>
pub const DJ_MIX_ALBUM_IRI_HTTP: &str = "http://schema.org/DJMixAlbum";
/// <https://schema.org/DJMixAlbum>
pub const DJ_MIX_ALBUM_IRI_HTTPS: &str = "https://schema.org/DJMixAlbum";
/// <https://schema.org/DJMixAlbum>
pub const DJ_MIX_ALBUM_LABEL: &str = "DJMixAlbum";
pub struct DjMixAlbumIri;
impl PartialEq<&str> for DjMixAlbumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DJ_MIX_ALBUM_IRI_HTTP || *other == DJ_MIX_ALBUM_IRI_HTTPS
	}
}
impl PartialEq<DjMixAlbumIri> for &str {
	fn eq(&self, other: &DjMixAlbumIri) -> bool {
		*self == DJ_MIX_ALBUM_IRI_HTTP || *self == DJ_MIX_ALBUM_IRI_HTTPS
	}
}
pub struct DjMixAlbumIriOrLabel;
impl PartialEq<&str> for DjMixAlbumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DjMixAlbumIri || *other == DJ_MIX_ALBUM_LABEL
	}
}
impl PartialEq<DjMixAlbumIriOrLabel> for &str {
	fn eq(&self, other: &DjMixAlbumIriOrLabel) -> bool {
		*self == DjMixAlbumIri || *self == DJ_MIX_ALBUM_LABEL
	}
}
