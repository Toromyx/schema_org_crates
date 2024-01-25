/// <https://schema.org/AlbumRelease>
pub const ALBUM_RELEASE_IRI_HTTP: &str = "http://schema.org/AlbumRelease";
/// <https://schema.org/AlbumRelease>
pub const ALBUM_RELEASE_IRI_HTTPS: &str = "https://schema.org/AlbumRelease";
/// <https://schema.org/AlbumRelease>
pub const ALBUM_RELEASE_LABEL: &str = "AlbumRelease";
pub struct AlbumReleaseIri;
impl PartialEq<&str> for AlbumReleaseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALBUM_RELEASE_IRI_HTTP || *other == ALBUM_RELEASE_IRI_HTTPS
	}
}
impl PartialEq<AlbumReleaseIri> for &str {
	fn eq(&self, other: &AlbumReleaseIri) -> bool {
		*self == ALBUM_RELEASE_IRI_HTTP || *self == ALBUM_RELEASE_IRI_HTTPS
	}
}
pub struct AlbumReleaseIriOrLabel;
impl PartialEq<&str> for AlbumReleaseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlbumReleaseIri || *other == ALBUM_RELEASE_LABEL
	}
}
impl PartialEq<AlbumReleaseIriOrLabel> for &str {
	fn eq(&self, other: &AlbumReleaseIriOrLabel) -> bool {
		*self == AlbumReleaseIri || *self == ALBUM_RELEASE_LABEL
	}
}
