/// <https://schema.org/MusicAlbumReleaseType>
pub const MUSIC_ALBUM_RELEASE_TYPE_IRI_HTTP: &str = "http://schema.org/MusicAlbumReleaseType";
/// <https://schema.org/MusicAlbumReleaseType>
pub const MUSIC_ALBUM_RELEASE_TYPE_IRI_HTTPS: &str = "https://schema.org/MusicAlbumReleaseType";
/// <https://schema.org/MusicAlbumReleaseType>
pub const MUSIC_ALBUM_RELEASE_TYPE_LABEL: &str = "MusicAlbumReleaseType";
pub struct MusicAlbumReleaseTypeIri;
impl PartialEq<&str> for MusicAlbumReleaseTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_ALBUM_RELEASE_TYPE_IRI_HTTP || *other == MUSIC_ALBUM_RELEASE_TYPE_IRI_HTTPS
	}
}
impl PartialEq<MusicAlbumReleaseTypeIri> for &str {
	fn eq(&self, other: &MusicAlbumReleaseTypeIri) -> bool {
		*self == MUSIC_ALBUM_RELEASE_TYPE_IRI_HTTP || *self == MUSIC_ALBUM_RELEASE_TYPE_IRI_HTTPS
	}
}
pub struct MusicAlbumReleaseTypeIriOrLabel;
impl PartialEq<&str> for MusicAlbumReleaseTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicAlbumReleaseTypeIri || *other == MUSIC_ALBUM_RELEASE_TYPE_LABEL
	}
}
impl PartialEq<MusicAlbumReleaseTypeIriOrLabel> for &str {
	fn eq(&self, other: &MusicAlbumReleaseTypeIriOrLabel) -> bool {
		*self == MusicAlbumReleaseTypeIri || *self == MUSIC_ALBUM_RELEASE_TYPE_LABEL
	}
}
