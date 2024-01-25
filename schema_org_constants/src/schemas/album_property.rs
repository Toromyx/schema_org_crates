/// <https://schema.org/album>
pub const ALBUM_PROPERTY_IRI_HTTP: &str = "http://schema.org/album";
/// <https://schema.org/album>
pub const ALBUM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/album";
/// <https://schema.org/album>
pub const ALBUM_PROPERTY_LABEL: &str = "album";
pub struct AlbumPropertyIri;
impl PartialEq<&str> for AlbumPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALBUM_PROPERTY_IRI_HTTP || *other == ALBUM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AlbumPropertyIri> for &str {
	fn eq(&self, other: &AlbumPropertyIri) -> bool {
		*self == ALBUM_PROPERTY_IRI_HTTP || *self == ALBUM_PROPERTY_IRI_HTTPS
	}
}
pub struct AlbumPropertyIriOrLabel;
impl PartialEq<&str> for AlbumPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlbumPropertyIri || *other == ALBUM_PROPERTY_LABEL
	}
}
impl PartialEq<AlbumPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AlbumPropertyIriOrLabel) -> bool {
		*self == AlbumPropertyIri || *self == ALBUM_PROPERTY_LABEL
	}
}
