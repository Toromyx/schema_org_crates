/// <https://schema.org/albumReleaseType>
pub const ALBUM_RELEASE_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/albumReleaseType";
/// <https://schema.org/albumReleaseType>
pub const ALBUM_RELEASE_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/albumReleaseType";
/// <https://schema.org/albumReleaseType>
pub const ALBUM_RELEASE_TYPE_PROPERTY_LABEL: &str = "albumReleaseType";
pub struct AlbumReleaseTypePropertyIri;
impl PartialEq<&str> for AlbumReleaseTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALBUM_RELEASE_TYPE_PROPERTY_IRI_HTTP
			|| *other == ALBUM_RELEASE_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AlbumReleaseTypePropertyIri> for &str {
	fn eq(&self, other: &AlbumReleaseTypePropertyIri) -> bool {
		*self == ALBUM_RELEASE_TYPE_PROPERTY_IRI_HTTP
			|| *self == ALBUM_RELEASE_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct AlbumReleaseTypePropertyIriOrLabel;
impl PartialEq<&str> for AlbumReleaseTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlbumReleaseTypePropertyIri || *other == ALBUM_RELEASE_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<AlbumReleaseTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AlbumReleaseTypePropertyIriOrLabel) -> bool {
		*self == AlbumReleaseTypePropertyIri || *self == ALBUM_RELEASE_TYPE_PROPERTY_LABEL
	}
}
