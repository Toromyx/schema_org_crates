/// <https://schema.org/MixtapeAlbum>
pub const MIXTAPE_ALBUM_IRI_HTTP: &str = "http://schema.org/MixtapeAlbum";
/// <https://schema.org/MixtapeAlbum>
pub const MIXTAPE_ALBUM_IRI_HTTPS: &str = "https://schema.org/MixtapeAlbum";
/// <https://schema.org/MixtapeAlbum>
pub const MIXTAPE_ALBUM_LABEL: &str = "MixtapeAlbum";
pub struct MixtapeAlbumIri;
impl PartialEq<&str> for MixtapeAlbumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MIXTAPE_ALBUM_IRI_HTTP || *other == MIXTAPE_ALBUM_IRI_HTTPS
	}
}
impl PartialEq<MixtapeAlbumIri> for &str {
	fn eq(&self, other: &MixtapeAlbumIri) -> bool {
		*self == MIXTAPE_ALBUM_IRI_HTTP || *self == MIXTAPE_ALBUM_IRI_HTTPS
	}
}
pub struct MixtapeAlbumIriOrLabel;
impl PartialEq<&str> for MixtapeAlbumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MixtapeAlbumIri || *other == MIXTAPE_ALBUM_LABEL
	}
}
impl PartialEq<MixtapeAlbumIriOrLabel> for &str {
	fn eq(&self, other: &MixtapeAlbumIriOrLabel) -> bool {
		*self == MixtapeAlbumIri || *self == MIXTAPE_ALBUM_LABEL
	}
}
