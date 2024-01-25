/// <https://schema.org/albumRelease>
pub const ALBUM_RELEASE_PROPERTY_IRI_HTTP: &str = "http://schema.org/albumRelease";
/// <https://schema.org/albumRelease>
pub const ALBUM_RELEASE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/albumRelease";
/// <https://schema.org/albumRelease>
pub const ALBUM_RELEASE_PROPERTY_LABEL: &str = "albumRelease";
pub struct AlbumReleasePropertyIri;
impl PartialEq<&str> for AlbumReleasePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALBUM_RELEASE_PROPERTY_IRI_HTTP || *other == ALBUM_RELEASE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AlbumReleasePropertyIri> for &str {
	fn eq(&self, other: &AlbumReleasePropertyIri) -> bool {
		*self == ALBUM_RELEASE_PROPERTY_IRI_HTTP || *self == ALBUM_RELEASE_PROPERTY_IRI_HTTPS
	}
}
pub struct AlbumReleasePropertyIriOrLabel;
impl PartialEq<&str> for AlbumReleasePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlbumReleasePropertyIri || *other == ALBUM_RELEASE_PROPERTY_LABEL
	}
}
impl PartialEq<AlbumReleasePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AlbumReleasePropertyIriOrLabel) -> bool {
		*self == AlbumReleasePropertyIri || *self == ALBUM_RELEASE_PROPERTY_LABEL
	}
}
